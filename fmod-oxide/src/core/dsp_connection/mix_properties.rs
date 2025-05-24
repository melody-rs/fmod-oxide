// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::{c_float, c_int};

use crate::DspConnection;
use crate::{FmodResultExt, Result};

impl DspConnection {
    /// Sets the connection's volume scale.
    pub fn set_mix(&self, volume: c_float) -> Result<()> {
        unsafe { FMOD_DSPConnection_SetMix(self.as_ptr(), volume).to_result() }
    }

    /// Retrieves the connection's volume scale.
    pub fn get_mix(&self) -> Result<c_float> {
        let mut volume = 0.0;
        unsafe { FMOD_DSPConnection_GetMix(self.as_ptr(), &raw mut volume).to_result()? };
        Ok(volume)
    }

    /// Sets a 2 dimensional pan matrix that maps the signal from input channels (columns) to output speakers (rows).
    ///
    /// Matrix element values can be below 0 to invert a signal and above 1 to amplify the signal.
    /// Note that increasing the signal level too far may cause audible distortion.
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
        unsafe {
            FMOD_DSPConnection_SetMixMatrix(
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
    /// Matrix element values can be below 0 to invert a signal and above 1 to amplify the signal.
    /// Note that increasing the signal level too far may cause audible distortion.
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
            FMOD_DSPConnection_GetMixMatrix(
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
