# Qubit Serde

> [!WARNING]
> **已弃用**
>
> 本 crate 和仓库已停止维护。crates.io 上所有已发布的 `qubit-serde`
> 版本均已 yank。Duration Serde 适配器已迁移到
> [`qubit-datatype`](https://github.com/qubit-ltd/rs-datatype) 0.9，并通过其
> `duration` feature 提供。

请将依赖替换为：

```toml
[dependencies]
qubit-datatype = { version = "0.9", default-features = false, features = ["duration"] }
```

适配器路径迁移如下：

- `qubit_serde::serde::duration_millis` → `qubit_datatype::serde::duration_millis`
- `qubit_serde::serde::duration_millis_with_unit` → `qubit_datatype::serde::duration_millis_with_unit`
- `qubit_serde::serde::duration_with_unit` → `qubit_datatype::serde::duration_with_unit`

以下内容作为历史文档保留，供现有用户参考。

[![Rust CI](https://github.com/qubit-ltd/rs-serde/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-serde/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-serde/coverage-badge.json)](https://qubit-ltd.github.io/rs-serde/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-serde.svg?color=blue)](https://crates.io/crates/qubit-serde)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

## 历史文档

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

- `duration_with_unit` 会选择能够精确表示 duration 的最大单位，输出如 `2min`、
  `2500ms`、`500µs` 或 `42ns`。
- 精确格式可以 round-trip 所有 `Duration`，包括 `Duration::MAX`。
- 反序列化接受带 `ns`、`us`、`µs`、`μs`、`ms`、`s`、`min`、`h`、`d` 的
  Strict 字符串；仅 Lenient 接受的 `m` 别名会被拒绝。
- 序列化输出首选的精确形式；反序列化接受文档定义的 Strict 语法，且不会隐式
  trim 输入。
- 无效单位、无效数字、小数值和溢出都会被拒绝。
- 直接调用解析函数会返回 `qubit_datatype::DurationParseError`。

### 带单位的舍入毫秒字符串

- `duration_millis_with_unit` 的序列化和反序列化都严格使用
  `<舍入后的毫秒>ms` wire 格式。
- 它使用半向上舍入，会有意丢失亚毫秒精度，适合展示和兼容配置格式。
- 在 `Duration::MAX` 附近，格式化结果会在
  `18446744073709551615999ms` 饱和；这是能够解析回 `Duration` 的最大整毫秒值。

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
qubit-serde = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
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

## 依赖项

运行时依赖：

- `serde`：提供序列化和反序列化集成。
- `qubit-datatype`：提供 Duration 单位、精确格式化和结构化解析错误。

若调用方需要直接匹配 `DurationParseError` 变体，应将 `qubit-datatype` 声明为
直接依赖，而不是依赖本 crate 的传递依赖。

## 相关项目

Qubit 旗下的更多 Rust 库发布在 GitHub 组织
[qubit-ltd](https://github.com/qubit-ltd)。

## 测试

```bash
# 使用默认 feature 集运行测试
cargo test

# 使用项目声明的全部 feature 运行测试
cargo test --all-features

# 运行项目 CI 检查
./ci-check.sh

# 检查代码覆盖率
./coverage.sh
```

## 许可证

Copyright (c) 2025 - 2026. Haixing Hu. All rights reserved.

本项目基于 Apache License 2.0 授权。完整许可证文本请参阅
[LICENSE](LICENSE)。

## 贡献

欢迎贡献。请遵循 Rust API 指南，及时更新公共 API 文档与测试，并在提交
Pull Request 前运行 `./align-ci.sh`格式化代码，运行`./ci-check.sh`对齐CI要求。

## 作者

**Haixing Hu** - *Qubit Co. Ltd.*

仓库地址：[https://github.com/qubit-ltd/rs-serde](https://github.com/qubit-ltd/rs-serde)
