//! rremu 测试 harness：构建 guest、加载 ELF、运行 Machine、断言结果。

mod outcome;

use std::path::PathBuf;
use std::process::Command;

use rremu_machine::{MachineBuilder, RunResult};
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
        let image = match load_elf(&bytes) {
            Ok(img) => img,
            Err(e) => {
                eprintln!("  [SKIP] {} — ELF parse: {e}", spec.name);
                failed += 1;
                continue;
            }
        };

        let mut builder = MachineBuilder::new()
            .uart(0x1000_0000)
            .clint(0x0200_0000)
            .load(image);
        if spec.fb == Some(true) {
            builder = builder.framebuffer(0x2000_0000, 640, 480);
        }
        let mut machine = builder.build().expect("machine build failed");
        // dump 保留最近 64 条事件
        machine.set_event_ring_capacity(64, 1024);

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
            (Expect::Fault(substr), RunResult::Faulted { reason }) => reason.contains(substr.as_str()),
            (Expect::Timeout, RunResult::Paused) => true,
            _ => false,
        };

        // UART 输出校验：取走缓冲并与期望串比较
        if ok {
            if let Some(want) = &spec.uart_expect {
                let got = machine.drain_uart();
                let got_str = String::from_utf8_lossy(&got);
                if got_str.as_ref() != want {
                    println!("  [FAIL] {} — UART output mismatch: expected {:?}, got {:?}", spec.name, want, got_str);
                    failed += 1;
                    continue;
                }
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
fn write_dump(machine: &rremu_machine::Machine, name: &str, elf: &PathBuf) {
    let dir = PathBuf::from("dumps");
    if let Err(e) = std::fs::create_dir_all(&dir) {
        eprintln!("  (dump: cannot create {}: {e})", dir.display());
        return;
    }
    let path = dir.join(format!("{name}.json"));
    match machine.dump(elf.to_str()).to_json() {
        Ok(json) => match std::fs::write(&path, json) {
            Ok(()) => println!("  dump written: {} (rremu-report {})", path.display(), path.display()),
            Err(e) => eprintln!("  (dump write failed: {e})"),
        },
        Err(e) => eprintln!("  (dump serialize failed: {e})"),
    }
}
