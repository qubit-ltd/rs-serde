# Qubit Serde

[![CircleCI](https://circleci.com/gh/qubit-ltd/rs-serde.svg?style=shield)](https://circleci.com/gh/qubit-ltd/rs-serde)
[![Coverage Status](https://coveralls.io/repos/github/qubit-ltd/rs-serde/badge.svg?branch=main)](https://coveralls.io/github/qubit-ltd/rs-serde?branch=main)
[![Crates.io](https://img.shields.io/crates/v/qubit-serde.svg?color=blue)](https://crates.io/crates/qubit-serde)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

Reusable serde adapters and utilities for Rust.

## Overview

Qubit Serde collects small serde adapters that are useful across Rust
libraries. It currently provides duration formats used by configuration and
retry-style libraries, and is intended to grow with other focused serde utility
modules.

## Design Goals

- **Focused adapters**: provide small, reusable `#[serde(with = "...")]` modules.
- **Shared conversion semantics**: reuse `qubit-datatype` conversion rules for supported scalar formats.
- **Configuration friendly**: support human-readable duration strings where useful.
- **Easy reuse**: make adapters available through a consistent `qubit_serde::serde::*` path.

## Features

### Duration as Milliseconds

- `duration_millis` serializes `std::time::Duration` as a whole millisecond `u64`.
- Deserialization accepts a non-negative `u64` millisecond count.
- Duration-to-millisecond conversion uses explicit `qubit-datatype` millisecond options.

### Duration with Units

- `duration_with_unit` serializes durations as strings such as `500ms`.
- Deserialization accepts strings with `ns`, `us`, `µs`, `μs`, `ms`, `s`, `m`, `h`, or `d`.
- Bare integer input is accepted as milliseconds for lenient configuration parsing.
- Duration-to-string conversion uses explicit `qubit-datatype` millisecond options.
- Invalid units, invalid numbers, fractional values, and overflows are rejected.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
qubit-serde = "0.1.0"
```

## Quick Start

### Duration with Unit Strings

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

### Duration as Milliseconds

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

## API Reference

- [`serde::duration_millis`](https://docs.rs/qubit-serde/latest/qubit_serde/serde/duration_millis/index.html) - duration as whole milliseconds.
- [`serde::duration_with_unit`](https://docs.rs/qubit-serde/latest/qubit_serde/serde/duration_with_unit/index.html) - duration as strings with supported time units.

## Testing & Code Coverage

This project maintains tests for serialization, deserialization, accepted
formats, invalid inputs, and overflow cases.

### Running Tests

```bash
# Run all tests
cargo test

# Run with coverage report
./coverage.sh

# Generate text format report
./coverage.sh text

# Run CI checks (format, clippy, test, coverage, audit)
./ci-check.sh
```

### Coverage Metrics

See [COVERAGE.md](COVERAGE.md) for detailed coverage statistics.

## Dependencies

Runtime dependencies:

- `qubit-datatype` for shared duration conversion semantics.
- `serde` for serialization and deserialization integration.
- `serde_json` for scalar value handling in lenient duration deserialization.

## License

Copyright (c) 2025 - 2026. Haixing Hu, Qubit Co. Ltd. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

See [LICENSE](LICENSE) for the full license text.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Guidelines

- Follow the Rust API guidelines.
- Keep adapters small, documented, and aligned with shared conversion semantics.
- Add tests for success paths, invalid input, and boundary conditions.
- Run `./ci-check.sh` before submitting PRs.

## Author

**Haixing Hu** - *Qubit Co. Ltd.*

## Related Projects

More Rust libraries from Qubit are published under the [qubit-ltd](https://github.com/qubit-ltd) organization on GitHub.

---

Repository: [https://github.com/qubit-ltd/rs-serde](https://github.com/qubit-ltd/rs-serde)
