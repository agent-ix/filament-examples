//! Bounded capture of an owned validation child under one caller-owned deadline.

use std::fmt;
use std::io::{Read, Seek, SeekFrom};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant};

const OUTPUT_LIMIT: u64 = 524_288;

/// Stable internal distinctions needed by the scenario graders.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CaptureCode {
    /// The caller's absolute deadline expired.
    AssertionTimeout,
    /// One standard stream exceeded the capture quota.
    OutputLimit,
    /// Starting, observing, reaping or reading the child failed.
    ToolFailure,
}

/// Capture failure with the PID only after owned-child cleanup succeeded.
#[derive(Debug)]
pub(crate) struct CaptureError {
    pub(crate) code: CaptureCode,
    pub(crate) reaped_child: Option<u32>,
    detail: Box<str>,
}

impl CaptureError {
    fn new(code: CaptureCode, detail: &str) -> Self {
        Self {
            code,
            reaped_child: None,
            detail: detail.into(),
        }
    }
}
impl fmt::Display for CaptureError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "{:?}: {}", self.code, self.detail)
    }
}
impl std::error::Error for CaptureError {}

/// Check the same absolute deadline before and after validation work.
pub(crate) fn before_deadline(deadline: Instant) -> Result<(), CaptureError> {
    if Instant::now() >= deadline {
        return Err(CaptureError::new(
            CaptureCode::AssertionTimeout,
            "the shared assertion deadline expired",
        ));
    }
    Ok(())
}

fn stop_with_failure(child: &mut Child, code: CaptureCode, detail: &str) -> CaptureError {
    let child_id = child.id();
    // The child may exit between observation and kill; wait still collects it.
    let _ = child.kill();
    if child.wait().is_err() {
        return CaptureError::new(
            CaptureCode::ToolFailure,
            "could not reap the owned validation child",
        );
    }
    CaptureError {
        code,
        reaped_child: Some(child_id),
        detail: detail.into(),
    }
}

/// Capture a child with a shared absolute deadline and 512 KiB per output stream.
///
/// File-backed capture avoids pipe-buffer deadlock. The child is polled for
/// deadline/output exhaustion and killed and reaped before either error returns.
/// Completed output reads also stop at the quota, including a concurrent write.
pub(crate) fn capture_command(
    command: &mut Command,
    deadline: Instant,
) -> Result<Output, CaptureError> {
    before_deadline(deadline)?;
    let mut stdout = tempfile::tempfile()
        .map_err(|_| CaptureError::new(CaptureCode::ToolFailure, "cannot create stdout capture"))?;
    let mut stderr = tempfile::tempfile()
        .map_err(|_| CaptureError::new(CaptureCode::ToolFailure, "cannot create stderr capture"))?;
    let out_handle = stdout
        .try_clone()
        .map_err(|_| CaptureError::new(CaptureCode::ToolFailure, "cannot attach stdout capture"))?;
    let err_handle = stderr
        .try_clone()
        .map_err(|_| CaptureError::new(CaptureCode::ToolFailure, "cannot attach stderr capture"))?;
    before_deadline(deadline)?;
    let mut child = command
        .stdin(Stdio::null())
        .stdout(out_handle)
        .stderr(err_handle)
        .spawn()
        .map_err(|_| {
            CaptureError::new(
                CaptureCode::ToolFailure,
                "owned validation command could not start",
            )
        })?;
    let status = loop {
        if Instant::now() >= deadline {
            return Err(stop_with_failure(
                &mut child,
                CaptureCode::AssertionTimeout,
                "owned child exceeded the shared assertion deadline",
            ));
        }
        for file in [&stdout, &stderr] {
            match file.metadata() {
                Ok(metadata) if metadata.len() <= OUTPUT_LIMIT => {}
                Ok(_) => {
                    return Err(stop_with_failure(
                        &mut child,
                        CaptureCode::OutputLimit,
                        "validation output exceeded its byte budget",
                    ));
                }
                Err(_) => {
                    return Err(stop_with_failure(
                        &mut child,
                        CaptureCode::ToolFailure,
                        "could not observe validation output size",
                    ));
                }
            }
        }
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) => std::thread::sleep(
                deadline
                    .saturating_duration_since(Instant::now())
                    .min(Duration::from_millis(5)),
            ),
            Err(_) => {
                return Err(stop_with_failure(
                    &mut child,
                    CaptureCode::ToolFailure,
                    "could not collect the owned validation child status",
                ));
            }
        }
    };
    let mut out = Vec::new();
    let mut err = Vec::new();
    for (file, bytes) in [(&mut stdout, &mut out), (&mut stderr, &mut err)] {
        before_deadline(deadline)?;
        file.seek(SeekFrom::Start(0)).map_err(|_| {
            CaptureError::new(
                CaptureCode::ToolFailure,
                "could not seek completed validation output",
            )
        })?;
        file.take(OUTPUT_LIMIT + 1)
            .read_to_end(bytes)
            .map_err(|_| {
                CaptureError::new(
                    CaptureCode::ToolFailure,
                    "could not read completed validation output",
                )
            })?;
        if !u64::try_from(bytes.len()).is_ok_and(|length| length <= OUTPUT_LIMIT) {
            return Err(CaptureError::new(
                CaptureCode::OutputLimit,
                "completed validation output exceeded its byte budget",
            ));
        }
    }
    before_deadline(deadline)?;
    Ok(Output {
        status,
        stdout: out,
        stderr: err,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Provenance: QSL-2493
    #[test]
    fn output_bounds_accept_the_limit_and_reject_excess_on_either_stream()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut at_limit = Command::new("head");
        at_limit.args(["-c", &OUTPUT_LIMIT.to_string(), "/dev/zero"]);
        let output = capture_command(&mut at_limit, Instant::now() + Duration::from_secs(5))?;
        assert!(output.status.success());
        assert_eq!(u64::try_from(output.stdout.len())?, OUTPUT_LIMIT);
        assert!(output.stderr.is_empty());
        for redirect in ["", " >&2"] {
            let mut excess = Command::new("sh");
            excess.args([
                "-c",
                &format!("exec head -c {} /dev/zero{redirect}", OUTPUT_LIMIT + 1),
            ]);
            let failure = capture_command(&mut excess, Instant::now() + Duration::from_secs(5))
                .expect_err("one excess byte must fail capture");
            assert_eq!(failure.code, CaptureCode::OutputLimit);
            if let Some(child_id) = failure.reaped_child {
                let alive = Command::new("/bin/kill")
                    .args(["-0", &child_id.to_string()])
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()?;
                assert!(
                    !alive.success(),
                    "an output-exhausted owned child must have exited"
                );
            }
        }
        Ok(())
    }
}
