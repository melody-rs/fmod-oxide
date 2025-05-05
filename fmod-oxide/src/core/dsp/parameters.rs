// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CString;
use std::ffi::{c_float, c_int};

use crate::{Dsp, DspParameterDataType, DspParameterDescription};

mod sealed {
    pub trait ReadSeal {}

    pub trait WriteSeal {}
}
pub trait ReadableParameter: sealed::ReadSeal + Sized {
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self>;

    // FIXME Strings are a max of FMOD_DSP_GETPARAM_VALUESTR_LENGTH so we don't need to heap allocate them
    fn get_parameter_string(dsp: Dsp, index: c_int) -> Result<Utf8CString>;
}

pub trait WritableParameter: sealed::WriteSeal + Sized {
    fn set_parameter(self, dsp: Dsp, index: c_int) -> Result<()>;
}

impl sealed::ReadSeal for bool {}
impl ReadableParameter for bool {
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        let dsp = dsp.inner.as_ptr();
        unsafe {
            let mut value = FMOD_BOOL::FALSE;
            FMOD_DSP_GetParameterBool(dsp, index, &raw mut value, std::ptr::null_mut(), 0)
                .to_result()?;
            Ok(value.into())
        }
    }

    fn get_parameter_string(dsp: Dsp, index: c_int) -> Result<Utf8CString> {
        let dsp = dsp.inner.as_ptr();
        let mut bytes = [0; FMOD_DSP_GETPARAM_VALUESTR_LENGTH as usize];
        unsafe {
            FMOD_DSP_GetParameterBool(
                dsp,
                index,
                std::ptr::null_mut(),
                bytes.as_mut_ptr().cast(),
                FMOD_DSP_GETPARAM_VALUESTR_LENGTH as i32,
            )
            .to_result()?;

            let string = Utf8CString::from_utf8_with_nul(bytes.to_vec()).unwrap();
            Ok(string)
        }
    }
}

impl sealed::WriteSeal for bool {}
impl WritableParameter for bool {
    fn set_parameter(self, dsp: Dsp, index: c_int) -> Result<()> {
        let dsp = dsp.inner.as_ptr();
        unsafe { FMOD_DSP_SetParameterBool(dsp, index, self.into()).to_result() }
    }
}

impl sealed::ReadSeal for c_int {}
impl ReadableParameter for c_int {
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        let dsp = dsp.inner.as_ptr();
        unsafe {
            let mut value = 0;
            FMOD_DSP_GetParameterInt(dsp, index, &raw mut value, std::ptr::null_mut(), 0)
                .to_result()?;
            Ok(value)
        }
    }

    fn get_parameter_string(dsp: Dsp, index: c_int) -> Result<Utf8CString> {
        let dsp = dsp.inner.as_ptr();
        let mut bytes = [0; FMOD_DSP_GETPARAM_VALUESTR_LENGTH as usize];
        unsafe {
            FMOD_DSP_GetParameterInt(
                dsp,
                index,
                std::ptr::null_mut(),
                bytes.as_mut_ptr().cast(),
                FMOD_DSP_GETPARAM_VALUESTR_LENGTH as i32,
            )
            .to_result()?;

            let string = Utf8CString::from_utf8_with_nul(bytes.to_vec()).unwrap();
            Ok(string)
        }
    }
}

impl sealed::WriteSeal for c_int {}
impl WritableParameter for c_int {
    fn set_parameter(self, dsp: Dsp, index: c_int) -> Result<()> {
        let dsp = dsp.inner.as_ptr();
        unsafe { FMOD_DSP_SetParameterInt(dsp, index, self).to_result() }
    }
}

impl sealed::ReadSeal for c_float {}
impl ReadableParameter for c_float {
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        let dsp = dsp.inner.as_ptr();
        unsafe {
            let mut value = 0.0;
            FMOD_DSP_GetParameterFloat(dsp, index, &raw mut value, std::ptr::null_mut(), 0)
                .to_result()?;
            Ok(value)
        }
    }

    fn get_parameter_string(dsp: Dsp, index: c_int) -> Result<Utf8CString> {
        let dsp = dsp.inner.as_ptr();
        let mut bytes = [0; FMOD_DSP_GETPARAM_VALUESTR_LENGTH as usize];
        unsafe {
            FMOD_DSP_GetParameterFloat(
                dsp,
                index,
                std::ptr::null_mut(),
                bytes.as_mut_ptr().cast(),
                FMOD_DSP_GETPARAM_VALUESTR_LENGTH as i32,
            )
            .to_result()?;

            let string = Utf8CString::from_utf8_with_nul(bytes.to_vec()).unwrap();
            Ok(string)
        }
    }
}

impl sealed::WriteSeal for c_float {}
impl WritableParameter for c_float {
    fn set_parameter(self, dsp: Dsp, index: c_int) -> Result<()> {
        let dsp = dsp.inner.as_ptr();
        unsafe { FMOD_DSP_SetParameterFloat(dsp, index, self).to_result() }
    }
}

/// The trait for data types which a can be read from a DSP parameter.
///
///
/// # Safety
///
/// Any type that implements this type must have the same layout as the data type the DSP expects.
/// **This is very important to get right**.
// TODO VERY IMPORTANT work out specific semantics (parameter type checking, for example)
pub unsafe trait ReadableDataParameter: Sized {
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self>;
}
/// The trait for data types which a can be written to a DSP parameter.
///
/// # Safety
///
/// Any type that implements this type must have the same layout as the data type the DSP expects.
/// **This is very important to get right**.
pub unsafe trait WritableDataParameter: Sized {
    fn set_parameter(self, dsp: Dsp, index: c_int) -> Result<()>;
}

impl<T> sealed::ReadSeal for T where T: ReadableDataParameter {}
impl<T> ReadableParameter for T
where
    T: ReadableDataParameter,
{
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        <Self as ReadableDataParameter>::get_parameter(dsp, index)
    }

    fn get_parameter_string(dsp: Dsp, index: c_int) -> Result<Utf8CString> {
        let dsp = dsp.inner.as_ptr();
        let mut bytes = [0; FMOD_DSP_GETPARAM_VALUESTR_LENGTH as usize];
        unsafe {
            FMOD_DSP_GetParameterData(
                dsp,
                index,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                bytes.as_mut_ptr().cast(),
                FMOD_DSP_GETPARAM_VALUESTR_LENGTH as i32,
            )
            .to_result()?;

            let string = Utf8CString::from_utf8_with_nul(bytes.to_vec()).unwrap();
            Ok(string)
        }
    }
}

impl<T> sealed::WriteSeal for T where T: WritableDataParameter {}
impl<T> WritableParameter for T
where
    T: WritableDataParameter,
{
    fn set_parameter(self, dsp: Dsp, index: c_int) -> Result<()> {
        <Self as WritableDataParameter>::set_parameter(self, dsp, index)
    }
}

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

    pub fn set_parameter<P: WritableParameter>(&self, index: c_int, parameter: P) -> Result<()> {
        parameter.set_parameter(*self, index)
    }

    pub fn get_parameter<P: ReadableParameter>(&self, index: c_int) -> Result<P> {
        P::get_parameter(*self, index)
    }

    pub fn get_parameter_string<P: ReadableParameter>(&self, index: c_int) -> Result<Utf8CString> {
        P::get_parameter_string(*self, index)
    }

    /// # Safety
    ///
    /// You must ensure that the provided T matches the size and layout as the specified DSP parameter.
    pub unsafe fn set_raw_parameter_data<T>(&self, data: &T, index: c_int) -> Result<()> {
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
            FMOD_DSP_GetParameterData(
                self.inner.as_ptr(),
                index,
                std::ptr::from_mut(data).cast(),
                size_of_val(data) as _,
                std::ptr::null_mut(),
                0,
            )
            .to_result()
        }
    }
}
