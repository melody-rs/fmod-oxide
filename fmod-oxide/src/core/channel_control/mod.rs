// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ptr::NonNull;

use fmod_sys::*;

mod callback;
mod dsp;
mod filtering;
mod general;
mod panning;
mod playback;
mod scheduling;
mod spatialization;
mod volume;
pub use callback::{ChannelControlCallback, ChannelControlType};

// FMOD's C API provides two versions of functions for channels: one that takes a `*mut FMOD_CHANNEL` and one that takes a `*mut FMOD_CHANNELGROUP`.
// The C++ API provides a base class `ChannelControl` that `Channel` and `ChannelGroup` inherits from.
// Seeing as we can cast from FMOD_CHANNELCONTROL to Channel* (in c++) we should be able to cast from FMOD_CHANNEL(GROUP) to FMOD_CHANNELCONTROL.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct ChannelControl {
    pub(crate) inner: NonNull<FMOD_CHANNELCONTROL>,
}

#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for ChannelControl {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for ChannelControl {}

impl ChannelControl {
    /// # Safety
    ///
    /// `value` must be a valid pointer either aquired from [`Self::as_ptr`] or FMOD.
    ///
    /// # Panics
    ///
    /// Panics if `value` is null.
    pub unsafe fn from_ffi(value: *mut FMOD_CHANNELCONTROL) -> Self {
        let inner = NonNull::new(value).unwrap();
        ChannelControl { inner }
    }

    /// Converts `self` into its raw representation.
    pub fn as_ptr(self) -> *mut FMOD_CHANNELCONTROL {
        self.inner.as_ptr()
    }
}

impl From<ChannelControl> for *mut FMOD_CHANNELCONTROL {
    fn from(value: ChannelControl) -> Self {
        value.inner.as_ptr()
    }
}
