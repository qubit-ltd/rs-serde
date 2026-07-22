# Qubit Serde

[![Rust CI](https://github.com/qubit-ltd/rs-serde/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-serde/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-serde/coverage-badge.json)](https://qubit-ltd.github.io/rs-serde/coverage/)
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
- **Stable conversion semantics**: keep duration wire formats explicit and self-contained.
- **Configuration friendly**: support human-readable duration strings where useful.
- **Easy reuse**: make adapters available through a consistent `qubit_serde::serde::*` path.

## Features

### Duration as Milliseconds

- `duration_millis` serializes `std::time::Duration` as a whole millisecond `u64`.
- Deserialization accepts a non-negative `u64` millisecond count.
- Duration-to-millisecond conversion uses half-up rounding, so half
  milliseconds round up.

### Exact Duration with Units

- `duration_with_unit` selects the largest unit that represents the duration
  exactly, producing values such as `2min`, `2500ms`, `500µs`, or `42ns`.
- Exact formatting round-trips every `Duration`, including `Duration::MAX`.
- Deserialization accepts strict strings with `ns`, `us`, `µs`, `μs`, `ms`,
  `s`, `min`, `h`, or `d`; the Lenient-only `m` alias is rejected.
- Serialization emits a preferred exact form, while deserialization accepts
  the documented strict grammar without implicitly trimming input.
- Invalid units, invalid numbers, fractional values, and overflows are rejected.
- Direct parsing returns `qubit_datatype::DurationParseError`.

### Rounded Milliseconds with a Unit

- `duration_millis_with_unit` uses the strict `<rounded-millis>ms` wire form
  for both serialization and deserialization.
- It uses half-up rounding and is intentionally lossy for sub-millisecond
  values, making it suitable for displays and compatibility configuration.
- Near `Duration::MAX`, formatting saturates at
  `18446744073709551615999ms`, the largest whole-millisecond value that can be
  parsed back into a `Duration`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
qubit-serde = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

## Quick Start

### Exact Duration with Unit Strings

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

### Rounded Millisecond Strings

Use `duration_millis_with_unit` when the wire format must remain fixed to
millisecond text:

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
- [`serde::duration_millis_with_unit`](https://docs.rs/qubit-serde/latest/qubit_serde/serde/duration_millis_with_unit/index.html) - duration as rounded millisecond text.
- [`serde::duration_with_unit`](https://docs.rs/qubit-serde/latest/qubit_serde/serde/duration_with_unit/index.html) - exact duration strings with automatically selected units.

## Dependencies

Runtime dependencies:

- `serde` for serialization and deserialization integration.
- `qubit-datatype` for duration units, exact formatting, and structured
  duration parsing errors.

Callers that match `DurationParseError` variants directly should also declare
`qubit-datatype` as a direct dependency instead of relying on this crate's
transitive dependency.

## Related Projects

More Rust libraries from Qubit are published under the
[qubit-ltd](https://github.com/qubit-ltd) organization on GitHub.

## Testing

```bash
# Run tests with the default feature set
cargo test

# Run tests with all declared features
cargo test --all-features

# Project CI checks
./ci-check.sh

# Check code coverage
./coverage.sh
```

## License

Copyright (c) 2025 - 2026. Haixing Hu. All rights reserved.

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for the
full license text.

## Contributing

Contributions are welcome. Please follow the Rust API guidelines, keep public
API documentation and tests current, and run `./align-ci.sh` to format code and
`./ci-check.sh` to satisfy CI requirements before submitting a pull request.

## Author

**Haixing Hu** - *Qubit Co. Ltd.*

Repository: [https://github.com/qubit-ltd/rs-serde](https://github.com/qubit-ltd/rs-serde)
