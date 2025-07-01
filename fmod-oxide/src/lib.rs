//! fmod-oxide
//! Safe rust bindings to the FMOD sound engine.
//! This crate tries to be as rusty and low-cost as possible, without compromising on any APIs.
//! Certain APIs, such as loading banks from a pointer, are marked as unsafe, but are still available for use.
//!
//! Supports FMOD versions >2.0.2.28 and >2.0.3.07, and Windows/Linux/MacOS/HTML5 platforms.
//!
//! Any newer patch-level FMOD versions should compile but might have missing features.
//!
//! ### Using this crate
//!
//! Due to licensing restrictions this crate can't bundle FMOD, so you'll need to [download](https://www.fmod.com/download) a copy of FMOD yourself.
//!
//! Make sure to download from `FMOD Engine` specifically.
//! ![Download page screenshot](https://github.com/melody-rs/fmod-oxide/blob/main/images/download_page.png?raw=true)
//!
//! After downloading FMOD, you have to tell this crate where FMOD is located.
//! **If you're on Windows and used the FMOD installer, you don't have to worry about this.**
//!
//! The easiest way is to create a cargo config in your project's root.
//!
//! ```toml
//! # `.cargo/config.toml`
//!
//! [env]
//! FMOD_SYS_FMOD_DIRECTORY = { value = "<absolute install path here>" }
//! ```
//!
//! You can also specify a relative install path like so:
//!
//! ```toml
//! # `.cargo/config.toml`
//!
//! [env]
//! FMOD_SYS_FMOD_DIRECTORY = { value = "<install path here>", relative = true }
//! ```
//!
//! (not recommended because rust-analyzer won't know this) Alternatively, you can specify `FMOD_SYS_FMOD_DIRECTORY` when building your project:
//!
//! `FMOD_SYS_FMOD_DIRECTORY=<path> cargo run`
//!
//! ### Cross compilation
//!
//! This crate supports cross compilation and will look for a target-specific FMOD install.
//!
//! The logic is quite basic at the moment, but it'll check if `<fmod install dir>/<target os>` exists and use that.
//!
//! If no target specific directory was found, it'll default to `<fmod install dir>`
//!
//! ### Using with webassembly
//!
//! Currently only `wasm32-unknown-emscripten` works well.
//! `wasm32-unknown-unknown` also works in some capacity but you have to essentially reimplement parts of libc and emscripten.
//!
//! Unfortunately `wasm-bindgen` doesn't work without patches right now, so your milage may vary
//!
//! The setup is roughly the same, except you'll need to add some arguments to `EMCC_FLAGS`.
//!
//! You can do this by editing `.cargo/config.toml`:
//! ```toml
//! # `.cargo/config.toml`
//!
//! [env]
//! EMCC_CFLAGS="-s EXPORTED_RUNTIME_METHODS=ccall,cwrap,setValue,getValue" # FMOD requires this
//! ```
//!
//! If you're using `wasm32-unknown-unknown`, you'll additionally need to add this until [this issue](https://github.com/rust-lang/rust/issues/138762) is closed.
//!
//! ```toml
//! # `.cargo/config.toml`
//!
//! [build]
//! rustflags="-Zwasm-c-abi=spec"
//! ```
//!
//! See [`web-examples/emscripten`](web-examples/emscripten) for a more detailed example.
//!
//! # Memory management & Copy types
//!
//! All FMOD objects are Copy, Clone, Send and Sync because it's possible to have multiple references to the same object. (e.g. loading a bank and then retrieving it by its path)
//! There are a lot of use-cases where you may want to fetch something (like a bank) and never use it again.
//! Implementing `Drop` to automatically release things would go against that particular use-case, so this crate opts to have manual `release()` methods instead.
//!
//! This crate does not currently guard against use-after-frees, *however* using most of FMOD's types (especially FMOD Studio's types) after calling `release()` is safe.
//! I'm still not 100% sure of what is and isn't safe and I'm actively trying to test this.
//!
//! # String types
//!
//! fmod-oxide aims to be as zero-cost as possible, and as such, it uses UTF-8 C strings from the `lanyard` crate as its string type.
//! This means that all FMOD functions take a `&Utf8CStr` instead of a `&str` or `&CStr`.
//! `&Utf8CStr` is pretty cheap to construct (and can even be done statically with the `c!` macro), so this should not be a problem
//!
//! When FMOD returns a string, it will always return a `Utf8CString` (the owned version of `Utf8CStr`) because it's difficult to encode lifetime requirements of FMOD strings.
//!
//! This applies to structs like `fmod::studio::AdvancedSettings` which store C strings.
//! Converting structs like `AdvancedSettings` to their FFI equivalents is done by reference as to not pass ownership of the string to FMOD
//!
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
#![cfg_attr(
    docsrs,
    feature(doc_auto_cfg, doc_cfg_hide),
    doc(cfg_hide(doc, docsrs))
)]
#![warn(
    rust_2018_idioms,
    clippy::pedantic,
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs,
    rustdoc::all
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

/// How much of FMOD's API fmod-oxide covers.
#[cfg(any(doc, doctest, test))]
pub mod coverage {
    pub mod _2_0_3 {
        #![doc = include_str!("../COVERAGE.2.03.md")]
        #[allow(unused_imports)]
        use fmod_sys::*;
    }
    pub mod _2_0_2 {
        #![doc = include_str!("../COVERAGE.2.02.md")]
        #[allow(unused_imports)]
        use fmod_sys::*;
    }
}

mod result;
pub(crate) use result::FmodResultExt;
pub use result::{Error, Result};

// Not really practical to go no_std.
// FMOD requires libc on pretty much every platform (even webassembly!)
// If you're using libc you probably can use std too.

/// The low-level FMOD core API.
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

mod owned;
pub use owned::Owned;

/// The FMOD Studio API.
///
/// The Studio API is a more high-level library which is tightly integrated with *FMOD Studio*, FMOD's production tool.
#[cfg(feature = "studio")]
pub mod studio;

/// Current FMOD version number.
///
/// The version is a 32 bit hexadecimal value formatted as 16:8:8, with the upper 16 bits being the product version,
/// the middle 8 bits being the major version and the bottom 8 bits being the minor version.
/// For example a value of `0x00010203` is equal to `1.02.03`.
pub const VERSION: u32 = fmod_sys::FMOD_VERSION;
/// The FMOD build number.
pub const BUILD_NUMBER: u32 = fmod_sys::FMOD_BUILDNUMBER;
/// Maximum number of channels per sample of audio supported by audio files, buffers, connections and [`Dsp`]s.
pub const MAX_CHANNEL_WIDTH: u32 = fmod_sys::FMOD_MAX_CHANNEL_WIDTH;
/// Maximum number of listeners supported.
pub const MAX_LISTENERS: u32 = fmod_sys::FMOD_MAX_LISTENERS;
/// The maximum number of global reverb instances.
///
/// Each instance of a reverb is an instance of an [`DspType::SfxReverb`] DSP in the DSP graph.
/// This is unrelated to the number of possible [`Reverb3D`] objects, which is unlimited.
pub const MAX_REVERB_INSTANCES: u32 = fmod_sys::FMOD_REVERB_MAXINSTANCES;
/// Maximum number of System objects allowed.
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
