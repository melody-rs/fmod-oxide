// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::ffi::c_void;

use crate::panic_wrapper;

use super::Dsp;
use crate::{FmodResultExt, Result};

/// Trait for this particular FMOD callback.
///
/// No `self` parameter is passed to the callback!
pub trait DspCallback {
    /// Called when a DSP's data parameter can be released.
    // I'm not sure how FMOD_DSP_DATA_PARAMETER_INFO works we'll just pass the raw value
    fn data_parameter_release(dsp: Dsp, info: FMOD_DSP_DATA_PARAMETER_INFO) -> Result<()>;
}

unsafe extern "C" fn callback_impl<C: DspCallback>(
    dsp: *mut FMOD_DSP,
    kind: FMOD_DSP_CALLBACK_TYPE,
    data: *mut c_void,
) -> FMOD_RESULT {
    panic_wrapper(|| {
        let dsp = unsafe { Dsp::from_ffi(dsp) };
        // FMOD may add more variants in the future, so keep the match for consistency
        #[allow(clippy::single_match_else)]
        let result = match kind {
            FMOD_DSP_CALLBACK_DATAPARAMETERRELEASE => {
                let info = unsafe { std::ptr::read(data.cast()) };
                C::data_parameter_release(dsp, info)
            }
            _ => {
                eprintln!("warning: unknown dsp callback type {kind}");
                return FMOD_RESULT::FMOD_OK;
            }
        };
        FMOD_RESULT::from_result(result)
    })
}

impl Dsp {
    /// Sets the callback for DSP notifications.
    pub fn set_callback<C: DspCallback>(&self) -> Result<()> {
        unsafe { FMOD_DSP_SetCallback(self.inner.as_ptr(), Some(callback_impl::<C>)).to_result() }
    }
}
