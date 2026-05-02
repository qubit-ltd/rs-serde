/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! Serde adapter for [`std::time::Duration`] as whole milliseconds.
//!
//! Serialization emits a `u64` millisecond value. Deserialization accepts a
//! `u64` millisecond value and converts it back to [`Duration`].

use std::time::Duration;

use serde::{
    Deserialize,
    Deserializer,
    Serializer,
};

/// Serializes a [`Duration`] as a saturated `u64` millisecond count.
///
/// # Parameters
/// - `duration`: Duration to serialize.
/// - `serializer`: Serde serializer receiving the millisecond count.
///
/// # Returns
/// The serializer result.
///
/// # Errors
/// Returns the serializer error if writing the integer value fails.
pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u64(as_millis_u64(duration))
}

/// Deserializes a [`Duration`] from a `u64` millisecond count.
///
/// # Parameters
/// - `deserializer`: Serde deserializer providing a millisecond count.
///
/// # Returns
/// A [`Duration`] with millisecond precision.
///
/// # Errors
/// Returns the deserializer error when the input is not a valid `u64`.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let millis = u64::deserialize(deserializer)?;
    Ok(Duration::from_millis(millis))
}

/// Converts a [`Duration`] to a saturated `u64` millisecond count.
///
/// # Parameters
/// - `duration`: Duration to convert.
///
/// # Returns
/// Whole milliseconds, saturated at [`u64::MAX`].
#[inline]
pub fn as_millis_u64(duration: &Duration) -> u64 {
    duration.as_millis().min(u128::from(u64::MAX)) as u64
}
