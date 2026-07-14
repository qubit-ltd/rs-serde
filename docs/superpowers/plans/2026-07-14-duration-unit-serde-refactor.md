# Duration Unit Serde Refactor Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Split fixed-millisecond and exact unit-bearing Duration adapters, replace `serde_json::Value` deserialization with a portable Visitor, add structured parse errors, and migrate real downstream users.

**Architecture:** `duration_with_unit` owns the exact formatter, parser, and shared Visitor; `parse_duration_error.rs` owns the public error type required by the repository's public-type layout rule. A new `duration_millis_with_unit` module owns only fixed-millisecond serialization/formatting and re-exports the common input API. Downstream display/config users select the fixed module while progress state keeps the exact module.

**Tech Stack:** Rust 2024, serde 1.0, qubit-datatype 0.5, serde_json tests, postcard 1.1 compatibility tests.

## Global Constraints

- Preserve all unrelated uncommitted changes in every repository.
- Do not run `git add`, `git commit`, or `git push`.
- Keep `duration_millis` wire format unchanged.
- Do not add a configurable output-unit API.
- Put Rust tests under `tests/`; do not add inline test modules.
- Every production behavior change must follow a red-green TDD cycle.

---

### Task 1: Restore a Testable Dependency Baseline

**Files:**
- Modify: `Cargo.toml`
- Mechanical update: `Cargo.lock`

**Interfaces:**
- Consumes: current local `qubit-datatype 0.5` converter API.
- Produces: an existing-test baseline on which feature tests can fail for the intended reason.

- [ ] **Step 1: Align the converter dependency**

Change the existing dependency line to:

```toml
qubit-datatype = { version = "0.5", features = ["converter"] }
```

- [ ] **Step 2: Run the existing test suite**

Run: `cargo test --all-features --all-targets`

Expected: the pre-refactor tests pass; any API mismatch is fixed without changing adapter behavior.

- [ ] **Step 3: Review the dependency-only diff**

Run: `git --no-pager diff -- Cargo.toml Cargo.lock`

Expected: only the required version/lock resolution changes appear.

### Task 2: Specify Exact Formatting and Structured Parse Errors

**Files:**
- Modify: `tests/serde/duration_with_unit_tests.rs`
- Modify: `src/serde/duration_with_unit.rs`
- Create: `src/serde/parse_duration_error.rs`
- Modify: `src/serde/mod.rs`

**Interfaces:**
- Produces: `pub enum ParseDurationError`, `format(&Duration) -> String`, and
  `parse(&str) -> Result<Duration, ParseDurationError>`.

- [ ] **Step 1: Write failing exact-format tests**

Add table assertions equivalent to:

```rust
for (duration, expected) in [
    (Duration::ZERO, "0ms"),
    (Duration::from_secs(120), "2m"),
    (Duration::from_millis(2500), "2500ms"),
    (Duration::from_micros(500), "500us"),
    (Duration::from_nanos(42), "42ns"),
] {
    assert_eq!(duration_with_unit::format(&duration), expected);
}
```

Add a `Duration::MAX` assertion that parses the formatted value back exactly.

- [ ] **Step 2: Write failing error-variant tests**

Assert exact variants:

```rust
assert_eq!(
    duration_with_unit::parse("12fortnights"),
    Err(ParseDurationError::UnsupportedUnit {
        unit: "fortnights".to_string(),
    }),
);
assert_eq!(
    duration_with_unit::parse("12.5s"),
    Err(ParseDurationError::InvalidSyntax),
);
assert_eq!(
    duration_with_unit::parse(&format!("{}s", u128::MAX)),
    Err(ParseDurationError::OutOfRange),
);
```

- [ ] **Step 3: Run tests and verify RED**

Run: `cargo test --test serde_tests duration_with_unit -- --nocapture`

Expected: failures show the old fixed-millisecond output and missing error type.

- [ ] **Step 4: Implement the minimal exact formatter and parser**

Implement `ParseDurationError` with `Debug`, `Clone`, `PartialEq`, `Eq`, `Display`, and
`Error`. Format by checking exact divisibility in `d/h/m/s/ms/us/ns` order, with a
special `0ms` case. Parse the canonical ASCII grammar, map unknown alphabetic suffixes
to `UnsupportedUnit`, and map numeric/conversion overflow to `OutOfRange`.

- [ ] **Step 5: Run tests and verify GREEN**

Run: `cargo test --test serde_tests duration_with_unit -- --nocapture`

Expected: all exact-format and error tests pass.

### Task 3: Add the Fixed-Millisecond Unit Adapter

**Files:**
- Create: `src/serde/duration_millis_with_unit.rs`
- Create: `tests/serde/duration_millis_with_unit_tests.rs`
- Modify: `src/serde/mod.rs`
- Modify: `tests/serde/mod.rs`

**Interfaces:**
- Consumes: `duration_millis::MILLISECOND_CONVERSION_OPTIONS` and the shared parser/Visitor.
- Produces: `serde::duration_millis_with_unit::{serialize, deserialize, format, parse,
  ParseDurationError}`.

- [ ] **Step 1: Write failing fixed-output tests**

Define a test holder using:

```rust
#[serde(with = "qubit_serde::serde::duration_millis_with_unit")]
duration: Duration,
```

Assert fixed output and half-up boundaries: `499us -> 0ms`, `500us -> 1ms`,
`1499us -> 1ms`, `1500us -> 2ms`.

- [ ] **Step 2: Run tests and verify RED**

Run: `cargo test --test serde_tests duration_millis_with_unit -- --nocapture`

Expected: compilation fails because the new module does not exist.

- [ ] **Step 3: Implement the fixed adapter**

Add the module export. Copy only the old fixed `serialize` and `format` behavior into the
new file, and publicly re-export `deserialize`, `parse`, and `ParseDurationError` from
`duration_with_unit`.

- [ ] **Step 4: Run tests and verify GREEN**

Run: `cargo test --test serde_tests duration_millis_with_unit -- --nocapture`

Expected: all fixed adapter tests pass.

### Task 4: Replace Value Deserialization with a Portable Visitor

**Files:**
- Modify: `Cargo.toml`
- Mechanical update: `Cargo.lock`
- Modify: `tests/serde/duration_with_unit_tests.rs`
- Modify: `src/serde/duration_with_unit.rs`

**Interfaces:**
- Produces: human-readable string/integer input and non-human-readable string round-trip.

- [ ] **Step 1: Add postcard as a dev dependency and write a failing binary round-trip test**

Add the maintained non-self-describing test dependency:

```toml
postcard = { version = "1.1", default-features = false, features = ["use-std"] }
```

Serialize and deserialize a holder containing `Duration::from_nanos(42)` with postcard,
then assert equality.

- [ ] **Step 2: Run the binary test and verify RED**

Run: `cargo test --test serde_tests test_duration_with_unit_postcard_round_trip -- --nocapture`

Expected: the old `serde_json::Value` path rejects postcard `deserialize_any`.

- [ ] **Step 3: Implement `DurationVisitor`**

Implement `visit_str`, `visit_string`, unsigned integer visits, and signed integer visits.
Use `deserialize_any` only when `is_human_readable()` is true; otherwise use
`deserialize_str`.

- [ ] **Step 4: Move serde_json to dev-dependencies**

The production code no longer imports `serde_json`; keep it only for integration tests.

- [ ] **Step 5: Run all rs-serde tests and verify GREEN**

Run: `cargo test --all-features --all-targets`

Expected: all adapter, JSON, and postcard tests pass.

### Task 5: Migrate Downstream Semantics

**Files:**
- Modify: `../rs-config/src/utils.rs`
- Modify: `../rs-retry/src/options/retry_delay_duration_format.rs`
- Modify: related `rs-retry` module docs and tests
- Modify: `../rs-progress/tests/model/progress_event_tests.rs`
- Modify: `../rs-progress/README.md`
- Modify: `../rs-progress/README.zh_CN.md`
- Modify: downstream `Cargo.toml`/`Cargo.lock` files only where version alignment is required

**Interfaces:**
- `rs-config` and `rs-retry` consume `duration_millis_with_unit`.
- `rs-progress` consumes exact `duration_with_unit`.

- [ ] **Step 1: Add a failing sub-millisecond progress round-trip test**

Construct an event with `elapsed = Duration::from_nanos(42)`, assert JSON contains
`"42ns"`, deserialize it, and assert exact equality.

- [ ] **Step 2: Run the progress test and verify RED against the old adapter**

Run with a local `qubit-serde` patch; expected old output is `0ms`, not `42ns`.

- [ ] **Step 3: Rename fixed downstream imports and documentation**

Change `rs-config` and `rs-retry` text formatting references from
`duration_with_unit` to `duration_millis_with_unit`. Keep progress source annotations on
`duration_with_unit`.

- [ ] **Step 4: Update progress documentation**

Replace rounding claims with exact automatic-unit behavior and examples.

- [ ] **Step 5: Run targeted downstream suites**

Run each crate with local dependency patches. Expected: all three suites pass and preserve
their unrelated dirty changes.

### Task 6: Documentation and Full Verification

**Files:**
- Modify: `README.md`
- Modify: `README.zh_CN.md`
- Review: all changed files in the four repositories

**Interfaces:**
- Produces: documented public contracts matching tested behavior.

- [ ] **Step 1: Update English and Chinese API documentation**

Document both adapters, exact-unit selection, fixed-millisecond lossiness, input grammar,
and `ParseDurationError`.

- [ ] **Step 2: Run formatting and style checks**

Run each affected crate's `cargo fmt --all -- --check` and project style checker where
available.

- [ ] **Step 3: Run lint and rustdoc checks**

Run `cargo clippy --all-features --all-targets -- -D warnings` and
`RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features` for `rs-serde`; run
proportional checks for downstream crates.

- [ ] **Step 4: Run fresh full test suites**

Run `cargo test --all-features --all-targets` for `rs-serde` and targeted/full suites for
all three downstream crates with local patches.

- [ ] **Step 5: Review final diffs and status**

Use `git --no-pager diff` and `git status --short` separately in every repository. Confirm
that no unrelated user change was overwritten and no Git staging/commit occurred.
