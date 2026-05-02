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
fn test_duration_millis_deserialize_from_integer() {
    let holder: Holder =
        serde_json::from_str(r#"{"duration":250}"#).expect("duration should deserialize");

    assert_eq!(holder.duration, Duration::from_millis(250));
}

#[test]
fn test_duration_millis_rejects_string() {
    let result = serde_json::from_str::<Holder>(r#"{"duration":"250ms"}"#);

    assert!(result.is_err());
}
