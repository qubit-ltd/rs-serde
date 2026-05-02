/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026 Haixing Hu.
 *
 *    SPDX-License-Identifier: Apache-2.0
 *
 *    Licensed under the Apache License, Version 2.0.
 *
 ******************************************************************************/
//! # Qubit Serde
//!
//! Provides reusable serde adapters and helper functions.
//!

#[path = "serde/mod.rs"]
mod serde_impl;

/// Reusable serde adapters.
pub mod serde {
    pub use super::serde_impl::*;
}
