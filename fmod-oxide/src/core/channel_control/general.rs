// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::c_void;

use crate::{ChannelControl, System};
use crate::{FmodResultExt, Result};

impl ChannelControl {
    /// Sets the user data.
    #[allow(clippy::not_unsafe_ptr_arg_deref)] // fmod doesn't dereference the passed in pointer, and the user dereferencing it is unsafe anyway
    pub fn set_userdata(&self, userdata: *mut c_void) -> Result<()> {
        unsafe { FMOD_ChannelControl_SetUserData(self.inner.as_ptr(), userdata).to_result() }
    }

    /// Retrieves user data.
    pub fn get_userdata(&self) -> Result<*mut c_void> {
        let mut userdata = std::ptr::null_mut();
        unsafe {
            FMOD_ChannelControl_GetUserData(self.inner.as_ptr(), &raw mut userdata).to_result()?;
        }
        Ok(userdata)
    }

    /// Retrieves the [`System`] that created this object.
    pub fn get_system(&self) -> Result<System> {
        let mut system = std::ptr::null_mut();
        unsafe {
            FMOD_ChannelControl_GetSystemObject(self.inner.as_ptr(), &raw mut system)
                .to_result()?;
            Ok(System::from_ffi(system))
        }
    }
}
