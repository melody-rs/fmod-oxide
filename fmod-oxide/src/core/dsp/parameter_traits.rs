use fmod_sys::*;
use lanyard::Utf8CString;
use std::ffi::{c_float, c_int};

use crate::{Dsp, DspType};

// FIXME don't want sealed so users can impl their own types, what do?

pub trait ReadableParameter: Sized {
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self>;

    // FIXME Strings are a max of FMOD_DSP_GETPARAM_VALUESTR_LENGTH so we don't need to heap allocate them
    fn get_parameter_string(dsp: Dsp, index: c_int) -> Result<Utf8CString>;
}

pub trait WritableParameter: Sized {
    fn set_parameter(self, dsp: Dsp, index: c_int) -> Result<()>;
}

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

impl WritableParameter for bool {
    fn set_parameter(self, dsp: Dsp, index: c_int) -> Result<()> {
        let dsp = dsp.inner.as_ptr();
        unsafe { FMOD_DSP_SetParameterBool(dsp, index, self.into()).to_result() }
    }
}

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

impl WritableParameter for c_int {
    fn set_parameter(self, dsp: Dsp, index: c_int) -> Result<()> {
        let dsp = dsp.inner.as_ptr();
        unsafe { FMOD_DSP_SetParameterInt(dsp, index, self).to_result() }
    }
}

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

impl<T> WritableParameter for T
where
    T: WritableDataParameter,
{
    fn set_parameter(self, dsp: Dsp, index: c_int) -> Result<()> {
        <Self as WritableDataParameter>::set_parameter(self, dsp, index)
    }
}

pub trait ReadableParameterIndex<T> {
    const TYPE: DspType;

    fn into_index(self) -> c_int;
}

impl<T> ReadableParameterIndex<T> for c_int {
    const TYPE: DspType = DspType::Unknown;

    fn into_index(self) -> c_int {
        self
    }
}

pub trait WritableParameterIndex<T> {
    const TYPE: DspType;

    fn into_index(self) -> c_int;
}

impl<T> WritableParameterIndex<T> for c_int {
    const TYPE: DspType = DspType::Unknown;

    fn into_index(self) -> c_int {
        self
    }
}
