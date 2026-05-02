/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Qubit Serde
//!
//! Provides reusable serde adapters and helper functions.
//!
//! # Author
//!
//! Haixing Hu

#[path = "serde/mod.rs"]
mod serde_impl;

/// Reusable serde adapters.
pub mod serde {
    pub use super::serde_impl::*;
}
