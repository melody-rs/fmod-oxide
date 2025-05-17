// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ptr::NonNull;

use fmod_sys::*;

mod callback;
mod channel_format;
mod connections;
mod data_parameters;
pub mod effects;
mod general;
mod metering;
mod parameter_traits;
mod parameters;
mod processing;

pub use callback::DspCallback;
pub use data_parameters::*;
pub use parameter_traits::*;

/// A digital signal processor is one node within a graph that transforms input audio signals into an output stream.
///
/// Create with [`System::createDSP`], [`System::createDSPByType`] or [`System::createDSPByPlugin`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct Dsp {
    pub(crate) inner: NonNull<FMOD_DSP>,
}

#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for Dsp {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for Dsp {}

impl Dsp {
    /// # Safety
    ///
    /// `value` must be a valid pointer either aquired from [`Self::as_ptr`] or FMOD.
    ///
    /// # Panics
    ///
    /// Panics if `value` is null.
    pub unsafe fn from_ffi(value: *mut FMOD_DSP) -> Self {
        let inner = NonNull::new(value).unwrap();
        Dsp { inner }
    }

    /// Converts `self` into its raw representation.
    pub fn as_ptr(self) -> *mut FMOD_DSP {
        self.inner.as_ptr()
    }
}

impl From<Dsp> for *mut FMOD_DSP {
    fn from(value: Dsp) -> Self {
        value.inner.as_ptr()
    }
}
