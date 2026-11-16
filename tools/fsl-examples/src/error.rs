//! The error type of the runner.

use std::path::Path;

/// What kind of failure an [`Error`] reports. Callers branch on this, never on the text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    /// An `example.toml` is malformed or breaks a manifest rule.
    Manifest,
    /// A file could not be read or written.
    Io,
    /// The FSL driver could not be started.
    Driver,
    /// A recorded file is unreadable or a README region names an unknown case.
    Recorded,
    /// The command line or a case reference is wrong.
    Usage,
}

/// A failure that stops a runner command. Case failures are not errors: they are counted and
/// reported line by line.
#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct Error {
    code: ErrorCode,
    message: String,
}

impl Error {
    /// A new error with its code and the message shown to the user.
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    /// The kind of failure.
    pub fn code(&self) -> ErrorCode {
        self.code
    }

    pub(crate) fn io(path: &Path, err: &std::io::Error) -> Self {
        Self::new(ErrorCode::Io, format!("{}: {err}", path.display()))
    }
}

/// The result type of the runner.
pub type Result<T> = std::result::Result<T, Error>;
