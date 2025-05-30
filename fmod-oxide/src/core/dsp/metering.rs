// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::mem::MaybeUninit;

use crate::{Dsp, DspMeteringInfo};
use crate::{FmodResultExt, Result};

impl Dsp {
    /// Retrieve the signal metering information.
    ///
    /// Requesting metering information when it hasn't been enabled will result in [`FMOD_RESULT::FMOD_ERR_BADCOMMAND`].
    ///
    /// `FMOD_INIT_PROFILE_METER_ALL` with `SystemBuilder::build` will automatically enable metering for all [`Dsp`] units.
    pub fn get_metering_info(&self) -> Result<(DspMeteringInfo, DspMeteringInfo)> {
        let mut input = MaybeUninit::zeroed();
        let mut output = MaybeUninit::zeroed();
        unsafe {
            FMOD_DSP_GetMeteringInfo(self.inner.as_ptr(), input.as_mut_ptr(), output.as_mut_ptr())
                .to_result()?;
            let input = input.assume_init().into();
            let output = output.assume_init().into();
            Ok((input, output))
        }
    }

    /// Sets the input and output signal metering enabled states.
    ///
    /// Input metering is pre processing, while output metering is post processing.
    ///
    /// Enabled metering allows [`Dsp::get_metering_info`] to return metering information and allows FMOD profiling tools to visualize the levels.
    ///
    /// `FMOD_INIT_PROFILE_METER_ALL` with `SystemBuilder::build` will automatically turn on metering for all [`Dsp`] units inside the mixer graph.
    ///
    /// This function must have inputEnabled and outputEnabled set to true if being used by the FMOD Studio API,
    /// such as in the Unity or Unreal Engine integrations, in order to avoid conflict with FMOD Studio's live update feature.
    pub fn set_metering_enabled(&self, input_enabled: bool, output_enabled: bool) -> Result<()> {
        unsafe {
            FMOD_DSP_SetMeteringEnabled(
                self.inner.as_ptr(),
                input_enabled.into(),
                output_enabled.into(),
            )
            .to_result()
        }
    }

    /// Retrieves the input and output signal metering enabled states.
    ///
    /// Input metering is pre processing, while output metering is post processing.
    ///
    /// Enabled metering allows [`Dsp::get_metering_info`] to return metering information and allows FMOD profiling tools to visualize the levels.
    ///
    /// `FMOD_INIT_PROFILE_METER_ALL` with `SystemBuilder::build` will automatically turn on metering for all [`Dsp`] units inside the mixer graph.
    pub fn get_metering_enabled(&self) -> Result<(bool, bool)> {
        let mut input_enabled = FMOD_BOOL::FALSE;
        let mut output_enabled = FMOD_BOOL::FALSE;
        unsafe {
            FMOD_DSP_GetMeteringEnabled(
                self.inner.as_ptr(),
                &raw mut input_enabled,
                &raw mut output_enabled,
            )
            .to_result()?;
        }
        Ok((input_enabled.into(), output_enabled.into()))
    }
}
