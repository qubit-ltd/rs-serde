/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! Tests for millisecond duration serde adapter.

use std::time::Duration;

use qubit_datatype::{
    DataConversionOptions,
    DataConverter,
    DurationConversionOptions,
    DurationUnit,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Holder {
    #[serde(with = "qubit_serde::serde::duration_millis")]
    duration: Duration,
}

#[test]
fn test_duration_millis_serialize_as_integer() {
    let holder = Holder {
        duration: Duration::from_millis(1500),
    };

    let json = serde_json::to_string(&holder).expect("duration should serialize");

    assert_eq!(json, r#"{"duration":1500}"#);
}

#[test]
fn test_duration_millis_serialize_uses_datatype_rounding() {
    let holder = Holder {
        duration: Duration::from_micros(1500),
    };

    let json = serde_json::to_string(&holder).expect("duration should serialize");

    assert_eq!(json, r#"{"duration":2}"#);
}

#[test]
fn test_duration_millis_serialize_pins_millisecond_options() {
    let seconds_options = DataConversionOptions::default().with_duration_options(
        DurationConversionOptions::default().with_unit(DurationUnit::Seconds),
    );
    let seconds: u64 = DataConverter::from(Duration::from_millis(2500))
        .to_with(&seconds_options)
        .expect("duration should convert to seconds");
    assert_eq!(seconds, 3);

    let holder = Holder {
        duration: Duration::from_millis(2500),
    };

    let json = serde_json::to_string(&holder).expect("duration should serialize");

    assert_eq!(json, r#"{"duration":2500}"#);
}

#[test]
fn test_duration_millis_serialize_rejects_out_of_range_millis() {
    let holder = Holder {
        duration: Duration::new(u64::MAX, 999_999_999),
    };

    let result = serde_json::to_string(&holder);

    assert!(result.is_err());
}

#[test]
fn test_duration_millis_deserialize_from_integer() {
    let holder: Holder =
        serde_json::from_str(r#"{"duration":250}"#).expect("duration should deserialize");

    assert_eq!(holder.duration, Duration::from_millis(250));
}

#[test]
fn test_duration_millis_deserialize_pins_millisecond_options() {
    let seconds_options = DataConversionOptions::default().with_duration_options(
        DurationConversionOptions::default().with_unit(DurationUnit::Seconds),
    );
    let seconds: Duration = DataConverter::from(2u64)
        .to_with(&seconds_options)
        .expect("integer should convert to duration seconds");
    assert_eq!(seconds, Duration::from_secs(2));

    let holder: Holder =
        serde_json::from_str(r#"{"duration":2}"#).expect("duration should deserialize");

    assert_eq!(holder.duration, Duration::from_millis(2));
}

#[test]
fn test_duration_millis_rejects_string() {
    let result = serde_json::from_str::<Holder>(r#"{"duration":"250ms"}"#);

    assert!(result.is_err());
}
