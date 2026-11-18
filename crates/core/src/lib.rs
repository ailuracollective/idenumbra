//! Core contract shared by every idenumbra country validator.
//!
//! This crate is intentionally dependency-free and `no_std`-compatible. It
//! defines the single [`Validator`] trait that every country crate implements,
//! the common [`ValidationError`] type, and the country code constants used
//! across the workspace.

#![no_std]

pub mod country;
pub mod error;
pub mod validator;

pub use country::*;
pub use error::ValidationError;
pub use validator::Validator;
