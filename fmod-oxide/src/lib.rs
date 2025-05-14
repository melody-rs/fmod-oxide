//! fmod-oxide
#![doc = include_str!("../../README.md")]
//! # Basic example
//! ```ignore
//! // System creation is unsafe and must be performed prior to any other FMOD operations.
//! let mut builder = unsafe { fmod::studio::SystemBuilder::new() }?;
//! let system = builder.build()?;
//!
//! // Load a bank
//! let bank = system.load_bank_file("path/to/bank.bank", fmod::studio::LoadBankFlags::NORMAL)?;
//! // Query all events in the bank
//! for event in bank.get_event_list().unwrap() {
//!     println!("Event: {}", event.get_path()?);
//! }
//!
//! // Releasing Systems is unsafe because it cannot be called concurrently, and all FMOD objects are rendered invalid.
//! unsafe { system.release() };
//! ```
//! # Feature flags
#![doc = document_features::document_features!()]
// Used to document cfgs (copied from https://docs.rs/winit/latest/src/winit/lib.rs.html#1-207)
#![cfg_attr(docsrs, feature(doc_auto_cfg), doc(cfg_hide(doc, docsrs)))]
#![warn(
    rust_2018_idioms,
    clippy::pedantic,
    missing_debug_implementations,
    missing_copy_implementations,
    //missing_docs
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::wildcard_imports,
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::must_use_candidate
)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![doc(html_favicon_url = "https://www.fmod.com/assets/fmod-logo.svg")]
#![doc(html_logo_url = "https://www.fmod.com/assets/fmod-logo.svg")]

mod result;
pub(crate) use result::FmodResultExt;
pub use result::{Error, Result};

// Not really practical to go no_std.
// FMOD requires libc on pretty much every platform (even webassembly!)
// If you're using libc you probably can use std too.

pub mod core;

#[doc(inline)]
pub use core::*;

#[doc(no_inline)]
pub use core::effects::*;
#[doc(no_inline)]
pub use fmod_sys as sys;
#[doc(no_inline)]
pub use lanyard::*;

#[cfg(test)]
mod tests;

/// The FMOD Studio API.
///
/// The Studio API is a more high-level library which is tightly integrated with *FMOD Studio*, FMOD's production tool.
#[cfg(feature = "studio")]
pub mod studio;

pub const VERSION: u32 = fmod_sys::FMOD_VERSION;
pub const MAX_CHANNEL_WIDTH: u32 = fmod_sys::FMOD_MAX_CHANNEL_WIDTH;
pub const MAX_LISTENERS: u32 = fmod_sys::FMOD_MAX_LISTENERS;
pub const MAX_SYSTEMS: u32 = fmod_sys::FMOD_MAX_SYSTEMS;

pub(crate) fn panic_wrapper<F>(f: F) -> fmod_sys::FMOD_RESULT
where
    F: FnOnce() -> fmod_sys::FMOD_RESULT,
    F: std::panic::UnwindSafe,
{
    let result = std::panic::catch_unwind(f);
    match result {
        Ok(r) => r,
        Err(e) => {
            print_panic_msg(&e);
            fmod_sys::FMOD_RESULT::FMOD_OK
        }
    }
}

pub(crate) fn print_panic_msg(msg: &dyn std::any::Any) {
    if let Some(str) = msg.downcast_ref::<&'static str>() {
        eprintln!("WARNING: caught {str}");
    } else if let Some(str) = msg.downcast_ref::<String>() {
        eprintln!("WARNING: caught {str}");
    } else {
        eprintln!("WARNING: caught panic!");
    }
}
