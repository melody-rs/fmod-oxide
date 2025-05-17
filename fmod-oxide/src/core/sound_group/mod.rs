// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ptr::NonNull;

use fmod_sys::*;

mod general;
mod group;
mod sound;

/// An interface that manages Sound Groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct SoundGroup {
    pub(crate) inner: NonNull<FMOD_SOUNDGROUP>,
}

#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for SoundGroup {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for SoundGroup {}

impl SoundGroup {
    /// # Safety
    ///
    /// `value` must be a valid pointer either aquired from [`Self::as_ptr`] or FMOD.
    ///
    /// # Panics
    ///
    /// Panics if `value` is null.
    pub unsafe fn from_ffi(value: *mut FMOD_SOUNDGROUP) -> Self {
        let inner = NonNull::new(value).unwrap();
        SoundGroup { inner }
    }

    /// Converts `self` into its raw representation.
    pub fn as_ptr(self) -> *mut FMOD_SOUNDGROUP {
        self.inner.as_ptr()
    }
}

impl From<SoundGroup> for *mut FMOD_SOUNDGROUP {
    fn from(value: SoundGroup) -> Self {
        value.inner.as_ptr()
    }
}
