#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::too_many_arguments)]
// FFI code frequently casts pointers for clarity even when redundant
#![allow(clippy::unnecessary_cast)]
// Transmute annotations are verbose for FFI function pointer casts
#![allow(clippy::missing_transmute_annotations)]
// Clamp-like patterns in FFI init code often have subtly different semantics
#![allow(clippy::manual_clamp)]
// Complex types are common in FFI function pointer signatures
#![allow(clippy::type_complexity)]
// Doc formatting lints — not worth the churn
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::doc_lazy_continuation)]

// Allow proc macros to reference `openwa_core::registry` even when
// invoked from within openwa-core itself (where the crate name is `crate`).
extern crate self as openwa_core;

// Re-export inventory so the define_addresses! macro can reference it
// from any crate that depends on openwa-core.
pub use inventory;

// Re-export derive macros so users write `use openwa_core::FieldRegistry;`
pub use openwa_derive::vtable;
pub use openwa_derive::FieldRegistry;

#[macro_use]
pub mod macros;

#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub mod address;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub mod asset;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub mod audio;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub mod bitgrid;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub mod engine;
pub mod field_format;
pub mod fixed;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub mod frontend;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub mod game;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub mod input;
pub mod log;
pub mod mem;
pub mod rebase;
pub mod registry;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub mod render;
pub mod rng;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub mod snapshot;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub mod task;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub mod trig;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub mod vtable;
#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub mod wa;
pub mod wa_alloc;
pub mod wa_call;
