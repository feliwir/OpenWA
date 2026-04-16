//! Typed wrappers for calling WA.exe and library functions.

pub mod frontend;
pub mod mfc;
#[cfg(target_os = "windows")]
pub mod registry;
pub mod resource;
