// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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

use crate::FmodResultExt;
use crate::owned::{HasRelease, Resource};

/// An instance of an FMOD Studio event.
#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)] // so we can transmute between types
#[allow(missing_copy_implementations)]
pub struct EventInstance {
    inner: std::marker::PhantomData<()>,
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
    pub unsafe fn from_ffi<'a>(value: *mut FMOD_STUDIO_EVENTINSTANCE) -> &'a Self {
        assert!(!value.is_null());
        unsafe { &*value.cast::<Self>() }
    }

    /// Converts `self` into its raw representation.
    pub fn as_ptr(&self) -> *mut FMOD_STUDIO_EVENTINSTANCE {
        std::ptr::from_ref(self).cast_mut().cast()
    }
}

impl From<&EventInstance> for *mut FMOD_STUDIO_EVENTINSTANCE {
    fn from(value: &EventInstance) -> Self {
        value.as_ptr()
    }
}

impl Resource for EventInstance {
    type Raw = FMOD_STUDIO_EVENTINSTANCE;

    fn from_raw<'a>(raw: std::ptr::NonNull<Self::Raw>) -> &'a Self {
        unsafe { &*raw.as_ptr().cast::<Self>() }
    }

    /// Marks the event instance for release.
    ///
    /// This function marks the event instance to be released.
    /// Event instances marked for release are destroyed by the asynchronous update when they are in the stopped state ([`PlaybackState::Stopped`]).
    ///
    /// Generally it is a best practice to release event instances immediately after calling [`EventInstance::start`],
    /// unless you want to play the event instance multiple times or explicitly stop it and start it again later.
    /// It is possible to interact with the instance after falling [`EventInstance::release`], however if the sound has stopped [`FMOD_RESULT::FMOD_ERR_INVALID_HANDLE`] will be returned.
    fn release(this: std::ptr::NonNull<Self::Raw>) -> crate::Result<()> {
        unsafe { FMOD_Studio_EventInstance_Release(this.as_ptr()) }.to_result()
    }
}
impl HasRelease for EventInstance {}
