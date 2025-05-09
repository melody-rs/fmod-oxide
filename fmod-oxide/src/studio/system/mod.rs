// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ptr::NonNull;

use fmod_sys::*;

mod bank;
mod builder;
mod callback;
mod command_replay;
mod general;
mod lifecycle;
mod listener;
mod misc;
mod parameter;
mod plugins;
mod profiling; // things too small to really make their own module

pub use bank::LoadBankUserdata;
pub use builder::SystemBuilder;
pub use callback::SystemCallback;

/// The main system object for FMOD Studio.
///
/// Initializing the FMOD Studio System object will also initialize the core System object.
///
/// Created with [`SystemBuilder`], which handles initialization for you.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct System {
    pub(crate) inner: NonNull<FMOD_STUDIO_SYSTEM>,
}

impl System {
    /// # Safety
    ///
    /// `value` must be a valid pointer either aquired from [`Self::as_ptr`] or FMOD.
    ///
    /// # Panics
    ///
    /// Panics if `value` is null.
    pub unsafe fn from_ffi(value: *mut FMOD_STUDIO_SYSTEM) -> Self {
        let inner = NonNull::new(value).unwrap();
        System { inner }
    }

    /// Converts `self` into its raw representation.
    pub fn as_ptr(self) -> *mut FMOD_STUDIO_SYSTEM {
        self.inner.as_ptr()
    }
}

/// Convert a System instance to its FFI equivalent.
///
/// This is safe, provided you don't use the pointer.
impl From<System> for *mut FMOD_STUDIO_SYSTEM {
    fn from(value: System) -> Self {
        value.inner.as_ptr()
    }
}

/// Most of FMOD is thread safe.
/// There are some select functions that are not thread safe to call, those are marked as unsafe.
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for System {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for System {}
