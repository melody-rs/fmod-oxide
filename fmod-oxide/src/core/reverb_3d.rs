// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    ffi::{c_float, c_void},
    mem::MaybeUninit,
    ptr::NonNull,
};

use fmod_sys::*;

use crate::{FmodResultExt, Result};
use crate::{ReverbProperties, Vector};

/// An interface that manages virtual 3D reverb spheres.
///
/// See the 3D Reverb Guide for more information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct Reverb3D {
    pub(crate) inner: NonNull<FMOD_REVERB3D>,
}

#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for Reverb3D {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for Reverb3D {}

impl Reverb3D {
    /// # Safety
    ///
    /// `value` must be a valid pointer either aquired from [`Self::as_ptr`] or FMOD.
    ///
    /// # Panics
    ///
    /// Panics if `value` is null.
    pub unsafe fn from_ffi(value: *mut FMOD_REVERB3D) -> Self {
        let inner = NonNull::new(value).unwrap();
        Reverb3D { inner }
    }

    /// Converts `self` into its raw representation.
    pub fn as_ptr(self) -> *mut FMOD_REVERB3D {
        self.inner.as_ptr()
    }
}

impl From<Reverb3D> for *mut FMOD_REVERB3D {
    fn from(value: Reverb3D) -> Self {
        value.inner.as_ptr()
    }
}

impl Reverb3D {
    /// Sets the 3D attributes of a reverb sphere.
    ///
    /// See the 3D Reverb guide for more information.
    ///
    /// When the position of the listener is less than `max_distance` away from the position of one or more reverb objects,
    /// the listener's 3D reverb properties are a weighted combination of those reverb objects.
    /// Otherwise, the reverb dsp will use the global reverb settings.
    pub fn set_3d_attributes(
        &self,
        position: Option<Vector>,
        min_distance: c_float,
        max_distance: c_float,
    ) -> Result<()> {
        let position = position
            .as_ref()
            .map_or(std::ptr::null(), std::ptr::from_ref)
            .cast();
        unsafe {
            FMOD_Reverb3D_Set3DAttributes(self.inner.as_ptr(), position, min_distance, max_distance)
                .to_result()
        }
    }

    /// Retrieves the 3D attributes of a reverb sphere.
    ///
    /// See the 3D Reverb guide for more information.
    pub fn get_3d_attributes(&self) -> Result<(Vector, c_float, c_float)> {
        let mut position = MaybeUninit::uninit();
        let mut min_distance = 0.0;
        let mut max_distance = 0.0;
        unsafe {
            FMOD_Reverb3D_Get3DAttributes(
                self.inner.as_ptr(),
                position.as_mut_ptr(),
                &raw mut min_distance,
                &raw mut max_distance,
            )
            .to_result()?;
            let position = position.assume_init().into();
            Ok((position, min_distance, max_distance))
        }
    }

    /// Sets the environmental properties of a reverb sphere.
    ///
    /// Reverb presets are available, see the associated constants of [`ReverbProperties`].
    pub fn set_properties(&self, properties: ReverbProperties) -> Result<()> {
        unsafe {
            FMOD_Reverb3D_SetProperties(self.inner.as_ptr(), std::ptr::from_ref(&properties).cast())
                .to_result()
        }
    }

    /// Retrieves the environmental properties of a reverb sphere.
    ///
    /// See the 3D Reverb guide for more information.
    pub fn get_properties(&self) -> Result<ReverbProperties> {
        let mut properties = MaybeUninit::uninit();
        unsafe {
            FMOD_Reverb3D_GetProperties(self.inner.as_ptr(), properties.as_mut_ptr())
                .to_result()?;
            let properties = properties.assume_init().into();
            Ok(properties)
        }
    }

    /// Sets the active state.
    ///
    /// See the 3D Reverb guide for more information.
    pub fn set_active(&self, active: bool) -> Result<()> {
        unsafe { FMOD_Reverb3D_SetActive(self.inner.as_ptr(), active.into()).to_result() }
    }

    /// Retrieves the active state.
    ///
    /// See the 3D Reverb guide for more information.
    pub fn get_active(&self) -> Result<bool> {
        let mut active = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_Reverb3D_GetActive(self.inner.as_ptr(), &raw mut active).to_result()?;
        }
        Ok(active.into())
    }

    /// Sets the user data.
    #[allow(clippy::not_unsafe_ptr_arg_deref)] // fmod doesn't dereference the passed in pointer, and the user dereferencing it is unsafe anyway
    pub fn set_userdata(&self, userdata: *mut c_void) -> Result<()> {
        unsafe { FMOD_Reverb3D_SetUserData(self.inner.as_ptr(), userdata).to_result() }
    }

    /// Retrieves user data.
    pub fn get_userdata(&self) -> Result<*mut c_void> {
        let mut userdata = std::ptr::null_mut();
        unsafe {
            FMOD_Reverb3D_GetUserData(self.inner.as_ptr(), &raw mut userdata).to_result()?;
        }
        Ok(userdata)
    }

    /// Releases the memory for a reverb object and makes it inactive.
    ///
    /// If you release all [`Reverb3D`] objects and have not added a new [`Reverb3D`] object,
    /// [`crate::System::set_reverb_properties`] should be called to reset the reverb properties.
    pub fn release(&self) -> Result<()> {
        unsafe { FMOD_Reverb3D_Release(self.inner.as_ptr()).to_result() }
    }
}
