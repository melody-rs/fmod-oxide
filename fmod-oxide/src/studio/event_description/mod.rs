// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ptr::NonNull;

use fmod_sys::*;

mod attributes;
mod callback;
mod general;
mod instance;
mod parameter;
mod sample_data;
mod user_property;

/// The description for an FMOD Studio event.
///
/// Event descriptions belong to banks, and so an event description can only be queried if the relevant bank is loaded.
/// Event descriptions may be retrieved via path or GUID lookup, or by enumerating all descriptions in a bank.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct EventDescription {
    pub(crate) inner: NonNull<FMOD_STUDIO_EVENTDESCRIPTION>,
}

#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for EventDescription {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for EventDescription {}

impl EventDescription {
    /// # Safety
    ///
    /// `value` must be a valid pointer either aquired from [`Self::as_ptr`] or FMOD.
    ///
    /// # Panics
    ///
    /// Panics if `value` is null.
    pub unsafe fn from_ffi(value: *mut FMOD_STUDIO_EVENTDESCRIPTION) -> Self {
        let inner = NonNull::new(value).unwrap();
        EventDescription { inner }
    }

    /// Converts `self` into its raw representation.
    pub fn as_ptr(self) -> *mut FMOD_STUDIO_EVENTDESCRIPTION {
        self.inner.as_ptr()
    }
}

impl From<EventDescription> for *mut FMOD_STUDIO_EVENTDESCRIPTION {
    fn from(value: EventDescription) -> Self {
        value.inner.as_ptr()
    }
}
