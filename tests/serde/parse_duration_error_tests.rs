// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Tests for the canonical Duration parsing error re-export.

use qubit_serde::serde::duration_with_unit::ParseDurationError;

/// Test stable Duration parsing error messages.
#[test]
fn test_parse_duration_error_display() {
    assert_eq!(
        ParseDurationError::InvalidSyntax.to_string(),
        "invalid duration syntax",
    );
    assert_eq!(
        ParseDurationError::UnsupportedUnit {
            unit: "fortnights".to_string(),
        }
        .to_string(),
        "unsupported duration unit `fortnights`",
    );
    assert_eq!(
        ParseDurationError::OutOfRange.to_string(),
        "duration value is out of range",
    );
}
