// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Lossy Serde adapter for [`std::time::Duration`] as millisecond text.
//!
//! Serialization rounds to the nearest whole millisecond using half-up
//! rounding and appends `ms`. Deserialization accepts canonical strings with
//! any supported duration unit, and human-readable formats also accept a bare
//! non-negative millisecond integer.

use std::time::Duration;

use serde::Serializer;

use super::duration_millis::rounded_millis;

pub use super::duration_with_unit::{
    deserialize,
    parse,
};

/// Serializes a [`Duration`] as rounded whole milliseconds with an `ms`
/// suffix.
///
/// # Parameters
///
/// - `duration`: Duration to round and serialize.
/// - `serializer`: Serde serializer receiving the formatted string.
///
/// # Returns
///
/// The serializer result.
///
/// # Errors
///
/// Returns the serializer error if writing the string value fails.
pub fn serialize<S>(
    duration: &Duration,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format(duration))
}

/// Formats a [`Duration`] as rounded whole milliseconds with an `ms` suffix.
///
/// # Parameters
///
/// - `duration`: Duration to round and format.
///
/// # Returns
///
/// A string in the form `<rounded-millis>ms`.
#[inline(always)]
pub fn format(duration: &Duration) -> String {
    let millis = rounded_millis(*duration);
    format!("{millis}ms")
}
