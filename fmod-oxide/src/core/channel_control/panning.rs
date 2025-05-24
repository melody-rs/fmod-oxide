// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::{c_float, c_int};

use fmod_sys::*;

use crate::ChannelControl;
use crate::{FmodResultExt, Result};

impl ChannelControl {
    /// Sets the left/right pan level.
    ///
    /// This is a convenience function to avoid passing a matrix, it will overwrite values set via `ChannelControl::set_mix_levels_input`,
    /// `ChannelControl::set_mix_levels_output` and `ChannelControl::set_mix_matrix`.
    ///
    /// Mono inputs are panned from left to right using constant power panning (non linear fade).
    ///  Stereo and greater inputs will isolate the front left and right input channels and fade them up and down based on the pan value (silencing other channels).
    /// The output channel count will always match the System speaker mode set via `SystemBuilder::software_format`.
    ///
    /// If the System is initialized with `FMOD_SPEAKERMODE_RAW` calling this function will produce silence.
    pub fn set_pan(&self, pan: c_float) -> Result<()> {
        unsafe { FMOD_ChannelControl_SetPan(self.as_ptr(), pan).to_result() }
    }

    /// Sets the incoming volume level for each channel of a multi-channel signal.
    ///
    /// This is a convenience function to avoid passing a matrix, it will overwrite values set via `ChannelControl::set_pan`,
    /// `ChannelControl::set_mix_levels_output` and `ChannelControl::set_mix_matrix`.
    ///
    /// #### NOTE: Currently only supported for Channel, not `ChannelGroup`.
    pub fn set_mix_levels_input(&self, levels: &mut [c_float]) -> Result<()> {
        // probably shouldn't be mutable but it's more safe that way?
        // FIXME do we need to enforce a max length?
        unsafe {
            FMOD_ChannelControl_SetMixLevelsInput(
                self.as_ptr(),
                levels.as_mut_ptr(),
                levels.len() as i32,
            )
            .to_result()
        }
    }

    /// Sets the outgoing volume levels for each speaker.
    ///
    /// Specify the level for a given output speaker, if the channel count of the input and output do not match,
    /// channels will be up/down mixed as appropriate to approximate the given speaker values.
    /// For example stereo input with 5.1 output will use the center parameter to distribute signal to the center speaker from front left and front right channels.
    ///
    /// This is a convenience function to avoid passing a matrix, it will overwrite values set via `ChannelControl::set_pan`, `ChannelControl::set_mix_levels_input` and `ChannelControl::set_mix_matrix`.
    ///
    /// The output channel count will always match the System speaker mode set via `SystemBuilder::software_format`.
    ///
    /// If the System is initialized with `FMOD_SPEAKERMODE_RAW` calling this function will produce silence.
    #[allow(clippy::too_many_arguments)] // no fixing this
    pub fn set_mix_levels_output(
        &self,
        front_left: c_float,
        front_right: c_float,
        center: c_float,
        lfe: c_float,
        surround_left: c_float,
        surround_right: c_float,
        back_left: c_float,
        back_right: c_float,
    ) -> Result<()> {
        unsafe {
            FMOD_ChannelControl_SetMixLevelsOutput(
                self.as_ptr(),
                front_left,
                front_right,
                center,
                lfe,
                surround_left,
                surround_right,
                back_left,
                back_right,
            )
            .to_result()
        }
    }

    // TODO i don't like this const generic API

    /// Sets a two-dimensional pan matrix that maps the signal from input channels (columns) to output speakers (rows).
    ///
    /// This will overwrite values set via [`ChannelControl::set_pan`], [`ChannelControl::set_mix_levels_input`] and [`ChannelControl::set_mix_levels_output`].
    pub fn set_mix_matrix<const IN: usize, const OUT: usize>(
        &self,
        matrix: [[f32; IN]; OUT],
    ) -> Result<()> {
        const {
            assert!(
                IN <= FMOD_MAX_CHANNEL_WIDTH as usize,
                "IN must be <= FMOD_MAX_CHANNEL_WIDTH"
            );
            assert!(
                OUT <= FMOD_MAX_CHANNEL_WIDTH as usize,
                "OUT must be <= FMOD_MAX_CHANNEL_WIDTH"
            );
        }
        // TODO: matrix can be null, cover that
        unsafe {
            FMOD_ChannelControl_SetMixMatrix(
                self.as_ptr(),
                matrix.as_ptr().cast::<f32>().cast_mut(),
                OUT as c_int,
                IN as c_int,
                IN as c_int,
            )
            .to_result()
        }
    }

    /// Retrieves a 2 dimensional pan matrix that maps the signal from input channels (columns) to output speakers (rows).
    ///
    /// Matrix element values can be below 0 to invert a signal and above 1 to amplify the signal. Note that increasing the signal level too far may cause audible distortion.
    pub fn get_mix_matrix<const IN: usize, const OUT: usize>(
        &self,
    ) -> Result<([[f32; IN]; OUT], c_int, c_int)> {
        const {
            assert!(
                IN <= FMOD_MAX_CHANNEL_WIDTH as usize,
                "IN must be <= FMOD_MAX_CHANNEL_WIDTH"
            );
            assert!(
                OUT <= FMOD_MAX_CHANNEL_WIDTH as usize,
                "OUT must be <= FMOD_MAX_CHANNEL_WIDTH"
            );
        }
        let mut matrix = [[0.0; IN]; OUT];
        let mut in_channels = IN as c_int;
        let mut out_channels = OUT as c_int;
        unsafe {
            FMOD_ChannelControl_GetMixMatrix(
                self.as_ptr(),
                matrix.as_mut_ptr().cast::<f32>(),
                &raw mut in_channels,
                &raw mut out_channels,
                IN as c_int,
            )
            .to_result()?;
        }
        Ok((matrix, in_channels, out_channels))
    }
}
