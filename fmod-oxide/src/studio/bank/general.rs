// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CString;
use std::ffi::c_void;
use std::mem::MaybeUninit;

use crate::Guid;
use crate::studio::{Bank, get_string_out_size};

impl Bank {
    /// Retrieves the GUID.
    pub fn get_id(&self) -> Result<Guid> {
        let mut guid = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_Bank_GetID(self.inner.as_ptr(), guid.as_mut_ptr()).to_result()?;

            let guid = guid.assume_init().into();

            Ok(guid)
        }
    }

    /// Retrieves the path.
    pub fn get_path(&self) -> Result<Utf8CString> {
        get_string_out_size(|path, size, ret| unsafe {
            FMOD_Studio_Bank_GetPath(self.inner.as_ptr(), path, size, ret)
        })
    }

    /// Checks that the Bank reference is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { FMOD_Studio_Bank_IsValid(self.inner.as_ptr()).into() }
    }

    /// Sets the bank's user data.
    ///
    /// This function allows arbitrary user data to be attached to this object.
    #[allow(clippy::not_unsafe_ptr_arg_deref)] // fmod doesn't dereference the passed in pointer, and the user dereferencing it is unsafe anyway
    pub fn set_userdata(&self, userdata: *mut c_void) -> Result<()> {
        unsafe { FMOD_Studio_Bank_SetUserData(self.inner.as_ptr(), userdata).to_result() }
    }

    /// Retrieves the bank's user data.
    ///
    /// This function allows arbitrary user data to be retrieved from this object.
    pub fn get_userdata(&self) -> Result<*mut c_void> {
        let mut userdata = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_Bank_GetUserData(self.inner.as_ptr(), &raw mut userdata).to_result()?;
        }
        Ok(userdata)
    }
}
