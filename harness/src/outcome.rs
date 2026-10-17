//! 测试规格与期望结果解析。

use serde::Deserialize;

/// 期望结果（三态断言 + 超时）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expect {
    /// 以 code=0 退出。
    Pass,
    /// 以非零 code 退出。
    Fail,
    /// 以 Faulted 终止，reason 包含 substr。
    Fault(String),
    /// 达到 max_instrs 仍未退出（step_n 返回 Paused）。
    Timeout,
}

/// Tests.toml 中一条 [test] 的反序列化结构。
#[derive(Debug, Clone, Deserialize)]
pub struct TestSpec {
    pub name: String,
    pub expect: String,
    pub max_instrs: u64,
    pub uart_preload: Option<String>,
    /// 运行结束后从 UART 输出缓冲取出的字节应与此字符串精确匹配。
    pub uart_expect: Option<String>,
    pub fb: Option<bool>,
    /// 覆盖启动 hartid（默认 0）。
    pub boot_hartid: Option<u32>,
    /// 覆盖启动 DTB 地址（默认不注入）。
    pub boot_dtb: Option<u32>,
    /// 将 ELF 段与入口整体平移到该基址。
    pub load_base: Option<u32>,
    /// 加载到 RAM 顶部的 DTB 字节（十六进制字符串），同时设置 boot a1。
    pub dtb_hex: Option<String>,
}

impl TestSpec {
    pub fn parse_expect(&self) -> Expect {
        match self.expect.as_str() {
            "Pass" => Expect::Pass,
            "Fail" => Expect::Fail,
            "Timeout" => Expect::Timeout,
            s if s.starts_with("Fault:") => {
                Expect::Fault(s["Fault:".len()..].to_string())
            }
            s => panic!("unknown expect: {s}"),
        }
    }
}

/// Tests.toml 顶层结构。
#[derive(Debug, Deserialize)]
pub struct Tests {
    pub test: Vec<TestSpec>,
}
