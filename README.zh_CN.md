# Qubit Serde

面向 Qubit Rust 项目的可复用 serde 适配器和辅助函数库。

`qubit-serde` 当前包含 duration 相关适配器，未来可以继续增加其他小型、可复用的 serde helper。

## 功能

- `duration_millis`：将 `std::time::Duration` 序列化为整毫秒数。
- `duration_with_unit`：将 duration 序列化为 `500ms` 这类字符串，并支持从带单位字符串或裸毫秒整数反序列化。

## 安装

```toml
[dependencies]
qubit-serde = "0.1.0"
```

## 快速开始

```rust
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    #[serde(with = "qubit_serde::serde::duration_with_unit")]
    timeout: Duration,
}
```

## 模块

- `serde::duration_millis`
- `serde::duration_with_unit`
