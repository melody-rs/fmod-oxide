// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ptr::NonNull;

use fmod_sys::*;

mod data_reading;
pub use data_reading::SoundLock;
mod defaults;
mod general;
mod information;
mod music;
mod relationship;
mod synchronization;
pub use synchronization::SyncPoint;

/// Container for sample data that can be played on a Channel.
///
/// Create with [`System::createSound`] or [`System::createStream`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct Sound {
    pub(crate) inner: NonNull<FMOD_SOUND>,
}

#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for Sound {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for Sound {}

impl Sound {
    /// # Safety
    ///
    /// `value` must be a valid pointer either aquired from [`Self::as_ptr`] or FMOD.
    ///
    /// # Panics
    ///
    /// Panics if `value` is null.
    pub unsafe fn from_ffi(value: *mut FMOD_SOUND) -> Self {
        let inner = NonNull::new(value).unwrap();
        Sound { inner }
    }

    /// Converts `self` into its raw representation.
    pub fn as_ptr(self) -> *mut FMOD_SOUND {
        self.inner.as_ptr()
    }
}

impl From<Sound> for *mut FMOD_SOUND {
    fn from(value: Sound) -> Self {
        value.inner.as_ptr()
    }
}
