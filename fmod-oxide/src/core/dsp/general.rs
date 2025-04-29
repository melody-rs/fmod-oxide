// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::{Utf8CStr, Utf8CString};
use std::ffi::{c_int, c_uint, c_void};

use crate::{Dsp, DspType, System};

#[derive(Debug)]
pub struct DspInfo {
    // FIXME: this is always 32 byes, it doesn't need to be heap allocated
    pub name: Utf8CString,
    pub version: c_uint,
    pub channels: c_int,
    pub config_width: c_int,
    pub config_height: c_int,
}

impl Dsp {
    /// Display or hide a DSP unit configuration dialog box inside the target window.
    ///
    /// Some DSP plug-ins (especially VST plug-ins) use dialog boxes to display graphical user interfaces for modifying their parameters,
    /// rather than using the other method of enumerating their parameters and setting them
    /// with [`Dsp::set_parameter`].
    ///
    /// To find out what size window to create to store the configuration screen, use [`Dsp::get_info`] where you can get the width and height.
    ///
    /// # Safety
    ///
    /// `hwnd` must be a valid window pointer.
    /// On Windows, this would be a `HWND`, on X11 a window id, etc.
    // FIXME Is that right?
    pub unsafe fn show_config_dialogue(&self, hwnd: *mut c_void, show: bool) -> Result<()> {
        unsafe { FMOD_DSP_ShowConfigDialog(self.inner.as_ptr(), hwnd, show.into()).to_result() }
    }

    /// Reset a DSPs internal state ready for new input signal.
    ///
    /// This will clear all internal state derived from input signal while retaining any set parameter values.
    /// The intended use of the function is to avoid audible artifacts if moving the [`Dsp`] from one part of the [`Dsp`] network to another.
    pub fn reset(&self) -> Result<()> {
        unsafe { FMOD_DSP_Reset(self.inner.as_ptr()).to_result() }
    }

    /// Frees a [`Dsp`] object.
    ///
    /// If [`Dsp`] is not removed from the network with `ChannelControl::removeDSP` after being added with `ChannelControl::addDSP`,
    /// it will not release and will instead return [`FMOD_RESULT::FMOD_ERR_DSP_INUSE`].
    pub fn release(self) -> Result<()> {
        unsafe { FMOD_DSP_Release(self.inner.as_ptr()).to_result() }
    }

    /// Retrieves the pre-defined type of a FMOD registered [`Dsp`] unit.
    pub fn get_type(&self) -> Result<DspType> {
        let mut dsp_type = 0;
        unsafe { FMOD_DSP_GetType(self.inner.as_ptr(), &raw mut dsp_type).to_result()? };
        let dsp_type = dsp_type.try_into()?;
        Ok(dsp_type)
    }

    /// Retrieves information about this DSP unit.
    pub fn get_info(&self) -> Result<DspInfo> {
        let mut buffer = [0u8; 32];
        let mut version = 0;
        let mut channels = 0;
        let mut config_width = 0;
        let mut config_height = 0;

        unsafe {
            FMOD_DSP_GetInfo(
                self.inner.as_ptr(),
                buffer.as_mut_ptr().cast::<i8>(),
                &raw mut version,
                &raw mut channels,
                &raw mut config_width,
                &raw mut config_height,
            )
            .to_result()?;
        }

        let name = Utf8CStr::from_utf8_with_nul(&buffer).unwrap().to_cstring();
        Ok(DspInfo {
            name,
            version,
            channels,
            config_width,
            config_height,
        })
    }

    /// Retrieves statistics on the mixer thread CPU usage for this unit.
    ///
    /// [`crate::InitFlags::PROFILE_ENABLE`] with [`crate::SystemBuilder::new`] is required to call this function.
    pub fn get_cpu_usage(&self) -> Result<(c_uint, c_uint)> {
        let mut exclusive = 0;
        let mut inclusive = 0;
        unsafe {
            FMOD_DSP_GetCPUUsage(self.inner.as_ptr(), &raw mut exclusive, &raw mut inclusive)
                .to_result()?;
        }
        Ok((exclusive, inclusive))
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)] // fmod doesn't dereference the passed in pointer, and the user dereferencing it is unsafe anyway
    pub fn set_userdata(&self, userdata: *mut c_void) -> Result<()> {
        unsafe { FMOD_DSP_SetUserData(self.inner.as_ptr(), userdata).to_result() }
    }

    pub fn get_userdata(&self) -> Result<*mut c_void> {
        let mut userdata = std::ptr::null_mut();
        unsafe {
            FMOD_DSP_GetUserData(self.inner.as_ptr(), &raw mut userdata).to_result()?;
        }
        Ok(userdata)
    }

    /// Retrieves the parent System object.
    pub fn get_system(&self) -> Result<System> {
        let mut system = std::ptr::null_mut();
        unsafe {
            FMOD_DSP_GetSystemObject(self.inner.as_ptr(), &raw mut system).to_result()?;
            Ok(System::from_ffi(system))
        }
    }
}
