// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{ffi::c_float, mem::MaybeUninit, ptr::NonNull};

use fmod_sys::*;
use lanyard::Utf8CString;

use crate::Guid;
use crate::{FmodResultExt, Result};

use super::get_string_out_size;

/// Represents a global mixer VCA.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct Vca {
    pub(crate) inner: NonNull<FMOD_STUDIO_VCA>,
}

#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for Vca {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for Vca {}

impl Vca {
    /// # Safety
    ///
    /// `value` must be a valid pointer either aquired from [`Self::as_ptr`] or FMOD.
    ///
    /// # Panics
    ///
    /// Panics if `value` is null.
    pub unsafe fn from_ffi(value: *mut FMOD_STUDIO_VCA) -> Self {
        let inner = NonNull::new(value).unwrap();
        Vca { inner }
    }

    /// Converts `self` into its raw representation.
    pub fn as_ptr(self) -> *mut FMOD_STUDIO_VCA {
        self.inner.as_ptr()
    }
}

impl From<Vca> for *mut FMOD_STUDIO_VCA {
    fn from(value: Vca) -> Self {
        value.inner.as_ptr()
    }
}

impl Vca {
    /// Sets the volume level.
    ///
    /// The VCA volume level is used to linearly modulate the levels of the buses and VCAs which it controls.
    pub fn set_volume(&self, volume: c_float) -> Result<()> {
        unsafe { FMOD_Studio_VCA_SetVolume(self.inner.as_ptr(), volume).to_result() }
    }

    /// Retrieves the volume level.
    ///
    /// The final combined volume returned in the second tuple field combines the user value set using [`Vca::set_volume`] with the result of any automation or modulation applied to the VCA.
    /// The final combined volume is calculated asynchronously when the Studio system updates.
    pub fn get_volume(&self) -> Result<(c_float, c_float)> {
        let mut volume = 0.0;
        let mut final_volume = 0.0;
        unsafe {
            FMOD_Studio_VCA_GetVolume(self.inner.as_ptr(), &raw mut volume, &raw mut final_volume)
                .to_result()?;
        }
        Ok((volume, final_volume))
    }
}

impl Vca {
    /// Retrieves the GUID.
    pub fn get_id(&self) -> Result<Guid> {
        let mut guid = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_VCA_GetID(self.inner.as_ptr(), guid.as_mut_ptr()).to_result()?;

            let guid = guid.assume_init().into();

            Ok(guid)
        }
    }

    /// Retrieves the path.
    ///
    /// The strings bank must be loaded prior to calling this function, otherwise [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned.
    // TODO: convert into possible macro for the sake of reusing code
    pub fn get_path(&self) -> Result<Utf8CString> {
        get_string_out_size(|path, size, ret| unsafe {
            FMOD_Studio_VCA_GetPath(self.inner.as_ptr(), path, size, ret)
        })
    }

    /// Checks that the VCA reference is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { FMOD_Studio_VCA_IsValid(self.inner.as_ptr()).into() }
    }
}
