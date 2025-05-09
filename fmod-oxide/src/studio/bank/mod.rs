// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ptr::NonNull;

use fmod_sys::*;

mod general;
mod loading;
mod lookups; // general lookups that are too small to be their own module

/// Banks made in FMOD Studio contain the metadata and audio sample data required for runtime mixing and playback.
///
/// Audio sample data may be packed into the same bank as the event metadata which references it, or it may be packed into separate banks.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct Bank {
    pub(crate) inner: NonNull<FMOD_STUDIO_BANK>,
}

#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for Bank {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for Bank {}

impl Bank {
    /// # Safety
    ///
    /// `value` must be a valid pointer either aquired from [`Self::as_ptr`] or FMOD.
    ///
    /// # Panics
    ///
    /// Panics if `value` is null.
    pub unsafe fn from_ffi(value: *mut FMOD_STUDIO_BANK) -> Self {
        let inner = NonNull::new(value).unwrap();
        Bank { inner }
    }

    /// Converts `self` into its raw representation.
    pub fn as_ptr(self) -> *mut FMOD_STUDIO_BANK {
        self.inner.as_ptr()
    }
}

impl From<Bank> for *mut FMOD_STUDIO_BANK {
    fn from(value: Bank) -> Self {
        value.inner.as_ptr()
    }
}
