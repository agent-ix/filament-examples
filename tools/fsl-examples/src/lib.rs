//! The runner of the filament-examples manifests.
//!
//! Each example system holds an `example.toml` with one entry per case. The runner runs each case
//! through the FSL driver and compares the outcome with the record in `expected/` by structure:
//! the exit code, the verdict, the category, the result value, the last stage, the code and cause
//! of the first diagnostic and the witness fields. Message text is kept in a separate snapshot
//! that `record` refreshes and that never fails a case.

pub mod error;
pub mod generated;
pub mod manifest;
pub mod outcome;
pub mod runner;

pub use error::{Error, ErrorCode, Result};
