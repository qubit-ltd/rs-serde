# Qubit Serde

Reusable serde adapters and helper functions for Qubit Rust projects.

`qubit-serde` currently contains duration adapters and is intended to grow with
other small, reusable serde helpers.

## Features

- `duration_millis`: serializes `std::time::Duration` as whole milliseconds.
- `duration_with_unit`: serializes durations as strings such as `500ms` and
  deserializes unit-suffixed strings or bare millisecond integers.

## Installation

```toml
[dependencies]
qubit-serde = "0.1.0"
```

## Quick Start

```rust
use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    #[serde(with = "qubit_serde::serde::duration_with_unit")]
    timeout: Duration,
}
```

## Module Layout

- `serde::duration_millis`
- `serde::duration_with_unit`
