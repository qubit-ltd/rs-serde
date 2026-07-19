# Qubit Serde

[![Rust CI](https://github.com/qubit-ltd/rs-serde/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-serde/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-serde/coverage-badge.json)](https://qubit-ltd.github.io/rs-serde/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-serde.svg?color=blue)](https://crates.io/crates/qubit-serde)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

面向 Rust 的可复用 serde 适配器和工具库。

## 概述

Qubit Serde 收集可在 Rust 库之间复用的小型 serde 适配器。当前提供配置和 retry 类库常用的 duration 格式，未来可以继续增加其他聚焦的 serde 工具模块。

## 设计目标

- **聚焦适配器**：提供小型、可复用的 `#[serde(with = "...")]` 模块。
- **稳定转换语义**：明确并在本 crate 内实现 duration 的 wire format。
- **配置友好**：在合适场景支持人类可读的 duration 字符串。
- **易于复用**：通过统一的 `qubit_serde::serde::*` 路径提供适配器。

## 特性

### Duration 作为毫秒数

- `duration_millis` 将 `std::time::Duration` 序列化为整毫秒 `u64`。
- 反序列化接受非负 `u64` 毫秒数。
- Duration 到毫秒数的转换使用半向上舍入，因此半毫秒会向上舍入。

### 精确的带单位 Duration 字符串

- `duration_with_unit` 会选择能够精确表示 duration 的最大单位，输出如 `2m`、
  `2500ms`、`500us` 或 `42ns`。
- 精确格式可以 round-trip 所有 `Duration`，包括 `Duration::MAX`。
- 反序列化接受带 `ns`、`us`、`ms`、`s`、`m`、`h`、`d` 的字符串。
- 裸整数输入会按毫秒处理，便于宽松配置解析。
- Duration 文本必须使用规范形式，不会被隐式 trim。
- 无效单位、无效数字、小数值和溢出都会被拒绝。
- 直接调用解析函数会返回 `qubit_datatype::DurationParseError`。

### 带单位的舍入毫秒字符串

- `duration_millis_with_unit` 始终序列化为 `<舍入后的毫秒>ms`。
- 它使用半向上舍入，会有意丢失亚毫秒精度，适合展示和兼容配置格式。
- 它与 `duration_with_unit` 共享反序列化和结构化解析语义。

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
qubit-serde = "0.4"
```

## 快速开始

### 精确的带单位 Duration 字符串

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
assert_eq!(json, r#"{"timeout":"5s"}"#);
```

### 带单位的舍入毫秒字符串

当 wire format 必须固定为毫秒文本时，使用 `duration_millis_with_unit`：

```rust
use std::time::Duration;

use serde::Serialize;

#[derive(Debug, Serialize)]
struct DisplayValue {
    #[serde(with = "qubit_serde::serde::duration_millis_with_unit")]
    elapsed: Duration,
}

let value = DisplayValue {
    elapsed: Duration::from_micros(1500),
};
let json = serde_json::to_string(&value).expect("duration should serialize");
assert_eq!(json, r#"{"elapsed":"2ms"}"#);
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
- [`serde::duration_millis_with_unit`](https://docs.rs/qubit-serde/latest/qubit_serde/serde/duration_millis_with_unit/index.html) - 将 duration 表示为带单位的舍入毫秒字符串。
- [`serde::duration_with_unit`](https://docs.rs/qubit-serde/latest/qubit_serde/serde/duration_with_unit/index.html) - 将 duration 表示为自动选择单位的精确字符串。

## 测试与代码覆盖率

本项目测试覆盖序列化、反序列化、精确与舍入语义、非自描述格式、无效输入和溢出场景。

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

- `serde`：提供序列化和反序列化集成。
- `thiserror`：提供结构化解析错误。

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
- 保持适配器小型、文档清楚，并明确说明转换语义。
- 为成功路径、无效输入和边界条件添加测试。
- 提交 PR 前运行 `./ci-check.sh`。

## 作者

**胡海星** - *Qubit Co. Ltd.*

## 相关项目

Qubit 旗下的更多 Rust 库发布在 GitHub 组织 [qubit-ltd](https://github.com/qubit-ltd)。

---

仓库地址：[https://github.com/qubit-ltd/rs-serde](https://github.com/qubit-ltd/rs-serde)
