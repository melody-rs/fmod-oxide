// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::owned::Resource;

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
#[derive(Debug, PartialEq, Eq, Hash)]
#[repr(transparent)] // so we can transmute between types
#[allow(missing_copy_implementations)]
pub struct System {
    inner: std::marker::PhantomData<()>,
}

/// Most of FMOD is thread safe.
/// There are some select functions that are not thread safe to call, those are marked as unsafe.
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
    pub unsafe fn from_ffi<'a>(value: *mut FMOD_STUDIO_SYSTEM) -> &'a Self {
        assert!(!value.is_null());
        unsafe { &*value.cast::<Self>() }
    }

    /// Converts `self` into its raw representation.
    pub fn as_ptr(&self) -> *mut FMOD_STUDIO_SYSTEM {
        std::ptr::from_ref(self).cast_mut().cast()
    }
}

impl From<&System> for *mut FMOD_STUDIO_SYSTEM {
    fn from(value: &System) -> Self {
        value.as_ptr()
    }
}

impl Resource for System {
    type Raw = FMOD_STUDIO_SYSTEM;

    fn from_raw<'a>(raw: std::ptr::NonNull<Self::Raw>) -> &'a Self {
        unsafe { &*raw.as_ptr().cast::<Self>() }
    }

    fn release(_: std::ptr::NonNull<Self::Raw>) -> crate::Result<()> {
        Ok(()) // no-op
    }
}
