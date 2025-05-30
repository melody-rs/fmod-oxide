// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::{c_float, c_int};

use fmod_sys::*;

use crate::ChannelGroup;
use crate::studio::EventInstance;
use crate::{FmodResultExt, Result};

impl EventInstance {
    /// Retrieves the core [`ChannelGroup`].
    ///
    /// Until the event instance has been fully created this function will return [`FMOD_RESULT::FMOD_ERR_STUDIO_NOT_LOADED`].
    pub fn get_channel_group(&self) -> Result<ChannelGroup> {
        let mut channel_group = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_EventInstance_GetChannelGroup(self.inner.as_ptr(), &raw mut channel_group)
                .to_result()?;
            Ok(ChannelGroup::from_ffi(channel_group))
        }
    }

    /// Sets the core reverb send level.
    ///          
    /// This function controls the send level for the signal from the event instance to a core reverb instance.
    pub fn set_reverb_level(&self, index: c_int, level: c_float) -> Result<()> {
        unsafe {
            FMOD_Studio_EventInstance_SetReverbLevel(self.inner.as_ptr(), index, level).to_result()
        }
    }

    /// Retrieves the core reverb send level.
    pub fn get_reverb_level(&self, index: c_int) -> Result<c_float> {
        let mut level = 0.0;
        unsafe {
            FMOD_Studio_EventInstance_GetReverbLevel(self.inner.as_ptr(), index, &raw mut level)
                .to_result()?;
        }
        Ok(level)
    }
}
