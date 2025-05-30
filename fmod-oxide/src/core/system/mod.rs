// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ptr::NonNull;

use fmod_sys::*;

mod builder;
mod callback;
mod creation;
mod device_selection;
mod filesystem;
mod general;
mod geometry;
mod information;
mod lifetime;
mod network;
mod plugin;
mod recording;
mod runtime_control;
mod setup;
pub use builder::SystemBuilder;
pub use callback::{ErrorCallbackInfo, Instance, SystemCallback, SystemCallbackMask};
pub use setup::RolloffCallback;

/// Management object from which all resources are created and played.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct System {
    pub(crate) inner: NonNull<FMOD_SYSTEM>,
}

#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for System {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for System {}

impl System {
    /// # Safety
    ///
    /// `value` must be a valid pointer either aquired from [`Self::as_ptr`] or FMOD.
    ///
    /// # Panics
    ///
    /// Panics if `value` is null.
    pub unsafe fn from_ffi(value: *mut FMOD_SYSTEM) -> Self {
        let inner = NonNull::new(value).unwrap();
        System { inner }
    }

    /// Converts `self` into its raw representation.
    pub fn as_ptr(self) -> *mut FMOD_SYSTEM {
        self.inner.as_ptr()
    }
}

impl From<System> for *mut FMOD_SYSTEM {
    fn from(value: System) -> Self {
        value.inner.as_ptr()
    }
}
