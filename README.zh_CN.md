# Qubit Serde

[![Rust CI](https://github.com/qubit-ltd/rs-serde/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-serde/actions/workflows/ci.yml)
[![Coverage Status](https://coveralls.io/repos/github/qubit-ltd/rs-serde/badge.svg?branch=main)](https://coveralls.io/github/qubit-ltd/rs-serde?branch=main)
[![Crates.io](https://img.shields.io/crates/v/qubit-serde.svg?color=blue)](https://crates.io/crates/qubit-serde)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

面向 Rust 的可复用 serde 适配器和工具库。

## 概述

Qubit Serde 收集可在 Rust 库之间复用的小型 serde 适配器。当前提供配置和 retry 类库常用的 duration 格式，未来可以继续增加其他聚焦的 serde 工具模块。

## 设计目标

- **聚焦适配器**：提供小型、可复用的 `#[serde(with = "...")]` 模块。
- **共享转换语义**：复用 `qubit-datatype` 对受支持标量格式的转换规则。
- **配置友好**：在合适场景支持人类可读的 duration 字符串。
- **易于复用**：通过统一的 `qubit_serde::serde::*` 路径提供适配器。

## 特性

### Duration 作为毫秒数

- `duration_millis` 将 `std::time::Duration` 序列化为整毫秒 `u64`。
- 反序列化接受非负 `u64` 毫秒数。
- Duration 到毫秒数的转换使用显式的 `qubit-datatype` 毫秒选项。

### 带单位的 Duration 字符串

- `duration_with_unit` 将 duration 序列化为 `500ms` 这样的字符串。
- 反序列化接受带 `ns`、`us`、`µs`、`μs`、`ms`、`s`、`m`、`h`、`d` 的字符串。
- 裸整数输入会按毫秒处理，便于宽松配置解析。
- Duration 到字符串的转换使用显式的 `qubit-datatype` 毫秒选项。
- 无效单位、无效数字、小数值和溢出都会被拒绝。

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
qubit-serde = "0.2"
```

## 快速开始

### 带单位字符串的 Duration

```rust
use std::time::Duration;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    #[serde(with = "qubit_serde::serde::duration_with_unit")]
    timeout: Duration,
}

let config: Config = serde_json::from_str(r#"{"timeout":"5s"}"#)
    .expect("duration with unit should parse");
assert_eq!(config.timeout, Duration::from_secs(5));

let json = serde_json::to_string(&config).expect("config should serialize");
assert_eq!(json, r#"{"timeout":"5000ms"}"#);
```

### 毫秒数形式的 Duration

```rust
use std::time::Duration;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
struct RetryState {
    #[serde(with = "qubit_serde::serde::duration_millis")]
    elapsed: Duration,
}

let state: RetryState = serde_json::from_str(r#"{"elapsed":250}"#)
    .expect("millisecond duration should parse");
assert_eq!(state.elapsed, Duration::from_millis(250));
```

## API 参考

- [`serde::duration_millis`](https://docs.rs/qubit-serde/latest/qubit_serde/serde/duration_millis/index.html) - 将 duration 表示为整毫秒数。
- [`serde::duration_with_unit`](https://docs.rs/qubit-serde/latest/qubit_serde/serde/duration_with_unit/index.html) - 将 duration 表示为带受支持时间单位的字符串。

## 测试与代码覆盖率

本项目测试覆盖序列化、反序列化、可接受格式、无效输入和溢出场景。

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行覆盖率报告
./coverage.sh

# 生成文本格式报告
./coverage.sh text

# 运行 CI 检查（格式化、clippy、测试、覆盖率、审计）
./ci-check.sh
```

### 覆盖率指标

详细的覆盖率统计请参见 [COVERAGE.zh_CN.md](COVERAGE.zh_CN.md)。

## 依赖项

运行时依赖：

- `qubit-datatype`：提供共享的 duration 转换语义。
- `serde`：提供序列化和反序列化集成。
- `serde_json`：用于宽松 duration 反序列化中的标量值处理。

## 许可证

Copyright (c) 2025 - 2026. Haixing Hu, Qubit Co. Ltd. All rights reserved.

根据 Apache 许可证 2.0 版（"许可证"）授权；
除非遵守许可证，否则您不得使用此文件。
您可以在以下位置获取许可证副本：

    http://www.apache.org/licenses/LICENSE-2.0

除非适用法律要求或书面同意，否则根据许可证分发的软件
按"原样"分发，不附带任何明示或暗示的担保或条件。
有关许可证下的特定语言管理权限和限制，请参阅许可证。

完整的许可证文本请参阅 [LICENSE](LICENSE)。

## 贡献

欢迎贡献！请随时提交 Pull Request。

### 开发指南

- 遵循 Rust API 指南。
- 保持适配器小型、文档清楚，并与共享转换语义对齐。
- 为成功路径、无效输入和边界条件添加测试。
- 提交 PR 前运行 `./ci-check.sh`。

## 作者

**胡海星** - *Qubit Co. Ltd.*

## 相关项目

Qubit 旗下的更多 Rust 库发布在 GitHub 组织 [qubit-ltd](https://github.com/qubit-ltd)。

---

仓库地址：[https://github.com/qubit-ltd/rs-serde](https://github.com/qubit-ltd/rs-serde)
