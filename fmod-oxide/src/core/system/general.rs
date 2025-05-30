// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::c_void;

use fmod_sys::*;

use crate::System;
use crate::{FmodResultExt, Result};

#[derive(Debug)]
pub struct DspLockGuard(System);

impl Drop for DspLockGuard {
    fn drop(&mut self) {
        let result = unsafe { FMOD_System_UnlockDSP(self.0.inner.as_ptr()).to_result() };
        if let Err(e) = result {
            eprintln!("FMOD_System_UnlockDSP errored! {e}");
        }
    }
}

impl System {
    /// Mutual exclusion function to lock the FMOD DSP engine (which runs asynchronously in another thread), so that it will not execute.
    ///
    /// If the FMOD DSP engine is already executing, this function will block until it has completed.
    ///
    /// The function may be used to synchronize DSP network operations carried out by the user.
    ///
    /// An example of using this function may be for when the user wants to construct a DSP sub-network, without the DSP engine executing in the background while the sub-network is still under construction.
    ///
    /// Note that the DSP engine should not be locked for a significant amount of time, otherwise inconsistency in the audio output may result. (audio skipping / stuttering).
    pub fn lock_dsp(&self) -> Result<DspLockGuard> {
        unsafe { FMOD_System_LockDSP(self.inner.as_ptr()).to_result()? };
        Ok(DspLockGuard(*self))
    }

    /// Sets the user data.
    #[allow(clippy::not_unsafe_ptr_arg_deref)] // fmod doesn't dereference the passed in pointer, and the user dereferencing it is unsafe anyway
    pub fn set_userdata(&self, userdata: *mut c_void) -> Result<()> {
        unsafe { FMOD_System_SetUserData(self.inner.as_ptr(), userdata).to_result() }
    }

    /// Retrieves user data.
    pub fn get_userdata(&self) -> Result<*mut c_void> {
        let mut userdata = std::ptr::null_mut();
        unsafe {
            FMOD_System_GetUserData(self.inner.as_ptr(), &raw mut userdata).to_result()?;
        }
        Ok(userdata)
    }
}
