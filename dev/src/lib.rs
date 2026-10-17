//! This repository's own gate steps (dev §G). One module per concern; the
//! binary in `main.rs` only reads files and maps verdicts to exit codes.

pub mod allowlist;
pub mod denylist;
pub mod finding;
pub mod message;
pub mod names;
pub mod refs;
