/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Serde adapter for [`std::time::Duration`] as whole milliseconds.
//!
//! Serialization emits a rounded `u64` millisecond value. Deserialization
//! accepts a `u64` millisecond value and converts it back to [`Duration`].

use std::time::Duration;

use qubit_datatype::DataConverter;
use serde::de::Error as DeserializeError;
use serde::ser::Error as SerializeError;
use serde::{
    Deserialize,
    Deserializer,
    Serializer,
};

/// Serializes a [`Duration`] as a rounded `u64` millisecond count.
///
/// # Parameters
/// - `duration`: Duration to serialize.
/// - `serializer`: Serde serializer receiving the millisecond count.
///
/// # Returns
/// The serializer result.
///
/// # Errors
/// Returns the serializer error if converting or writing the integer value
/// fails.
pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let millis = DataConverter::from(*duration)
        .to::<u64>()
        .map_err(S::Error::custom)?;
    serializer.serialize_u64(millis)
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
/// Returns the deserializer error when the input is not a valid `u64` or cannot
/// be converted to [`Duration`].
pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let millis = u64::deserialize(deserializer)?;
    DataConverter::from(millis)
        .to::<Duration>()
        .map_err(D::Error::custom)
}
