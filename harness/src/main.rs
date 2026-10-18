//! rremu 测试 harness：构建 guest、加载 ELF、运行 Machine、断言结果。

mod outcome;

use std::path::{Path, PathBuf};
use std::process::Command;

use rremu_machine::{Machine, RunResult};
use rremu_loader::load_elf;

use outcome::{Expect, Tests};

fn main() {
    let workspace_root = PathBuf::from("..");
    let guest_dir = workspace_root.join("guest");
    let tests_toml = guest_dir.join("Tests.toml");

    let content = std::fs::read_to_string(&tests_toml)
        .unwrap_or_else(|e| { eprintln!("cannot read {}: {e}", tests_toml.display()); std::process::exit(2) });
    let tests: Tests = toml::from_str(&content)
        .unwrap_or_else(|e| { eprintln!("parse Tests.toml: {e}"); std::process::exit(2) });
    let specs = tests.test;

    // 构建 guest
    println!("Building guest...");
    let build_status = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(&guest_dir)
        .status()
        .expect("failed to spawn cargo build");
    if !build_status.success() {
        eprintln!("guest build failed");
        std::process::exit(2);
    }

    let elf_dir = workspace_root
        .join("target/riscv32imac-unknown-none-elf/release");

    let mut passed = 0;
    let mut failed = 0;

    for spec in &specs {
        let elf_path = elf_dir.join(&spec.name);
        let bytes = match std::fs::read(&elf_path) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("  [SKIP] {} — ELF not found: {e}", spec.name);
                failed += 1;
                continue;
            }
        };
        let mut image = match load_elf(&bytes) {
            Ok(img) => img,
            Err(e) => {
                eprintln!("  [SKIP] {} — ELF parse: {e}", spec.name);
                failed += 1;
                continue;
            }
        };

        if let Some(base) = spec.load_base {
            image.relocate(base);
        }

        // 统一配置：默认布局（uart/clint/fb）；非 fb 用例不映射 framebuffer
        let mut cfg = rremu_config::Config::default();
        if spec.fb != Some(true) {
            cfg = cfg.with_fb(None);
        }
        if let Some(hartid) = spec.boot_hartid {
            cfg = cfg.with_boot_hartid(hartid);
        }
        if let Some(dtb) = spec.boot_dtb {
            cfg = cfg.with_boot_dtb_addr(Some(dtb));
        }
        let dtb_bytes = spec.dtb_hex.as_ref().map(|hex| {
            parse_hex(hex).unwrap_or_else(|e| panic!("{}: invalid dtb_hex: {e}", spec.name))
        });
        let cfg = if dtb_bytes.is_some() {
            cfg.with_boot_dtb()
                .unwrap_or_else(|e| panic!("{}: DTB placement failed: {e}", spec.name))
        } else {
            cfg
        };
        let dtb = dtb_bytes.map(|bytes| (cfg.boot().dtb.expect("DTB address missing"), bytes));

        let mut machine = Machine::from_config(&cfg, Some(image)).expect("machine build failed");

        if let Some((addr, bytes)) = &dtb {
            machine.write_mem(*addr, bytes).unwrap_or_else(|e| panic!("{}: DTB load failed: {e}", spec.name));
        }

        if let Some(input) = &spec.uart_preload {
            for byte in input.bytes() {
                machine.uart_push_input(byte);
            }
        }

        let result = machine.step_n(spec.max_instrs);
        let expect = spec.parse_expect();

        let ok = match (&expect, &result) {
            (Expect::Pass, RunResult::Exited { code: 0 }) => true,
            (Expect::Fail, RunResult::Exited { code }) if *code != 0 => true,
            (Expect::Exit(want), RunResult::Exited { code }) if code == want => true,
            (Expect::Fault(substr), RunResult::Faulted { reason }) => reason.contains(substr.as_str()),
            (Expect::Timeout, RunResult::Paused) => true,
            _ => false,
        };

        // UART 输出校验：取走缓冲并与期望串比较
        if ok && let Some(want) = &spec.uart_expect {
            let got = machine.drain_uart();
            let got_str = String::from_utf8_lossy(&got);
            if got_str.as_ref() != want {
                println!("  [FAIL] {} — UART output mismatch: expected {:?}, got {:?}", spec.name, want, got_str);
                failed += 1;
                continue;
            }
        }

        if ok {
            println!("  [PASS] {}", spec.name);
            passed += 1;
        } else {
            println!("  [FAIL] {} — expected {:?}, got {:?}", spec.name, expect, result);
            write_dump(&machine, &spec.name, &elf_path);
            failed += 1;
        }
    }

    println!("\n{} passed, {} failed", passed, failed);
    if failed > 0 {
        std::process::exit(1);
    }
}

/// 失败用例写出机器状态 dump（`dumps/<name>.json`），供 rremu-report 分析。
fn write_dump(machine: &rremu_machine::Machine, name: &str, elf: &Path) {
    let dir = PathBuf::from("dumps");
    if let Err(e) = std::fs::create_dir_all(&dir) {
        eprintln!("  (dump: cannot create {}: {e})", dir.display());
        return;
    }
    let path = dir.join(format!("{name}.json"));
    let elf_ref = elf.to_str().map(rremu_machine::dump::ElfRef::from_path);
    match machine.dump(elf_ref).to_json() {
        Ok(json) => match std::fs::write(&path, json) {
            Ok(()) => println!("  dump written: {} (rremu-report {})", path.display(), path.display()),
            Err(e) => eprintln!("  (dump write failed: {e})"),
        },
        Err(e) => eprintln!("  (dump serialize failed: {e})"),
    }
}

/// 解析十六进制字节串（允许空白分隔）。
fn parse_hex(s: &str) -> Result<Vec<u8>, String> {
    let compact: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    if !compact.len().is_multiple_of(2) {
        return Err("odd number of hex digits".into());
    }
    (0..compact.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&compact[i..i + 2], 16).map_err(|e| e.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    /// 跨仓库一致性护栏：`rremu-am/arch.rs`（人工镜像）必须与
    /// `rremu-config` 的默认值一致；漂移会被此测试立即抓住。
    #[test]
    fn config_defaults_match_am_arch() {
        use rremu_config::*;
        assert_eq!(rremu_am::arch::RAM_BASE, DEFAULT_RAM_BASE);
        assert_eq!(rremu_am::arch::RAM_SIZE, DEFAULT_RAM_SIZE);
        assert_eq!(rremu_am::arch::UART_BASE, DEFAULT_UART_BASE);
        assert_eq!(rremu_am::arch::CLINT_BASE, DEFAULT_CLINT_BASE);
        assert_eq!(rremu_am::arch::PLIC_BASE, DEFAULT_PLIC_BASE);
        assert_eq!(rremu_am::arch::SYSCON_BASE, DEFAULT_SYSCON_BASE);
        assert_eq!(rremu_am::arch::RTC_BASE, DEFAULT_RTC_BASE);
        assert_eq!(rremu_am::arch::FB_BASE, DEFAULT_FB_BASE);
        assert_eq!(rremu_am::arch::FB_WIDTH, DEFAULT_FB_WIDTH);
        assert_eq!(rremu_am::arch::FB_HEIGHT, DEFAULT_FB_HEIGHT);
    }
}

