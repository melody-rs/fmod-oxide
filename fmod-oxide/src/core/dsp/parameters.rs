// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CString;
use std::ffi::c_int;

use crate::{
    Dsp, DspParameterDataType, DspParameterDescription, ReadableParameter, ReadableParameterIndex,
    WritableParameter,
};

use super::WritableParameterIndex;

impl Dsp {
    /// Retrieve the index of the first data parameter of a particular data type.
    ///
    /// This function returns [`Ok`] if a parmeter of matching type is found and [`FMOD_RESULT::FMOD_ERR_INVALID_PARAM`] if no matches were found.
    ///
    /// The return code can be used to check whether the [`Dsp`] supports specific functionality through data parameters of certain types without the need to provide index.
    pub fn get_data_parameter_index(&self, data_type: DspParameterDataType) -> Result<c_int> {
        let mut index = 0;
        unsafe {
            FMOD_DSP_GetDataParameterIndex(self.inner.as_ptr(), data_type.into(), &raw mut index)
                .to_result()?;
        }
        Ok(index)
    }

    /// Retrieves the number of parameters exposed by this unit.
    ///
    /// Use this to enumerate all parameters of a [`Dsp`] unit with [`Dsp::get_parameter_info`].
    pub fn get_parameter_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe { FMOD_DSP_GetNumParameters(self.inner.as_ptr(), &raw mut count).to_result()? };
        Ok(count)
    }

    /// Retrieve information about a specified parameter.
    // FIXME do we keep this around?
    pub fn get_parameter_info(&self, index: c_int) -> Result<DspParameterDescription> {
        let mut desc = std::ptr::null_mut();
        unsafe {
            FMOD_DSP_GetParameterInfo(self.inner.as_ptr(), index, &raw mut desc).to_result()?;
            let desc = DspParameterDescription::from_ffi(*desc);
            Ok(desc)
        }
    }

    /// Retrieve information about a specified parameter.
    ///
    /// Returns the raw struct, useful if you don't want to pay for the expensive pointer copies
    /// that converting a [`FMOD_DSP_PARAMETER_DESC`] to a [`DspParameterDescription`] would entail.
    pub fn get_raw_parameter_info(&self, index: c_int) -> Result<FMOD_DSP_PARAMETER_DESC> {
        let mut desc = std::ptr::null_mut();
        unsafe {
            FMOD_DSP_GetParameterInfo(self.inner.as_ptr(), index, &raw mut desc).to_result()?;
            Ok(*desc)
        }
    }

    pub fn set_parameter<I, P>(&self, index: I, parameter: P) -> Result<()>
    where
        I: WritableParameterIndex<P>,
        P: WritableParameter,
    {
        parameter.set_parameter(*self, index.into_index())
    }

    pub fn get_parameter<I, P>(&self, index: I) -> Result<P>
    where
        I: ReadableParameterIndex<P>,
        P: ReadableParameter,
    {
        P::get_parameter(*self, index.into_index())
    }

    pub fn get_parameter_string<P, I>(&self, index: I) -> Result<Utf8CString>
    where
        I: ReadableParameterIndex<P>,
        P: ReadableParameter,
    {
        P::get_parameter_string(*self, index.into_index())
    }

    /// # Safety
    ///
    /// You must ensure that the provided T matches the size and layout as the specified DSP parameter.
    pub unsafe fn set_raw_parameter_data<T: ?Sized>(&self, data: &T, index: c_int) -> Result<()> {
        unsafe {
            FMOD_DSP_SetParameterData(
                self.inner.as_ptr(),
                index,
                std::ptr::from_ref(data).cast_mut().cast(),
                size_of_val(data) as _,
            )
            .to_result()
        }
    }

    /// # Safety
    ///
    /// You must ensure that the provided T matches the size and layout as the specified DSP parameter.
    pub unsafe fn get_raw_parameter_data<T>(
        &self,
        data: &mut std::mem::MaybeUninit<T>,
        index: c_int,
    ) -> Result<()> {
        unsafe {
            let mut data_ptr = std::ptr::null_mut();
            let mut data_size = 0;

            FMOD_DSP_GetParameterData(
                self.inner.as_ptr(),
                index,
                &raw mut data_ptr,
                &raw mut data_size,
                std::ptr::null_mut(),
                0,
            )
            .to_result()?;

            debug_assert_eq!(data_size, size_of::<T>() as _); // If this panics, we're in *trouble*

            std::ptr::copy(data_ptr.cast(), data.as_mut_ptr(), 1);

            Ok(())
        }
    }

    /// # Safety
    ///
    /// The returned slice has an effectively unbounded lifetime.
    /// You must copy it to an owned type (i.e. Vec) as soon as possible.
    pub unsafe fn get_raw_parameter_data_slice(&self, index: c_int) -> Result<&[u8]> {
        unsafe {
            // Can this be null?
            let mut data_ptr = std::ptr::null_mut();
            let mut data_size = 0;

            FMOD_DSP_GetParameterData(
                self.inner.as_ptr(),
                index,
                &raw mut data_ptr,
                &raw mut data_size,
                std::ptr::null_mut(),
                0,
            )
            .to_result()?;

            Ok(std::slice::from_raw_parts(data_ptr.cast(), data_size as _))
        }
    }
}
