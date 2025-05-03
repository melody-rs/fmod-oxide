// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ptr::NonNull;

use fmod_sys::*;

mod attributes_3d;
mod callback;
mod core;
mod general;
mod parameters;
mod playback;
mod playback_properties;
mod profiling;

pub use callback::EventInstanceCallback;
pub(crate) use callback::event_callback_impl;

/// An instance of an FMOD Studio event.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct EventInstance {
    pub(crate) inner: NonNull<FMOD_STUDIO_EVENTINSTANCE>,
}

#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for EventInstance {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for EventInstance {}

impl EventInstance {
    /// # Safety
    ///
    /// `value` must be a valid pointer either aquired from [`Self::as_ptr`] or FMOD.
    ///
    /// # Panics
    ///
    /// Panics if `value` is null.
    pub unsafe fn from_ffi(value: *mut FMOD_STUDIO_EVENTINSTANCE) -> Self {
        let inner = NonNull::new(value).unwrap();
        EventInstance { inner }
    }

    /// Converts `self` into its raw representation.
    pub fn as_ptr(self) -> *mut FMOD_STUDIO_EVENTINSTANCE {
        self.inner.as_ptr()
    }
}

impl From<EventInstance> for *mut FMOD_STUDIO_EVENTINSTANCE {
    fn from(value: EventInstance) -> Self {
        value.inner.as_ptr()
    }
}
