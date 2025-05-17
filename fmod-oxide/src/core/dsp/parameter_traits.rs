use fmod_sys::*;
use lanyard::{Utf8CStr, Utf8CString};
use std::ffi::{c_float, c_int};

use crate::{Dsp, DspType};
use crate::{FmodResultExt, Result};

// FIXME don't want sealed so users can impl their own types, what do?

/// Trait for types that can be read from DSP parameters.
///
/// You should either defer to [`Dsp::get_parameter`] or call [`FMOD_DSP_GetParameterData`].
///
/// # Data types
///
/// Implementing this trait for anything aside from data types is relatively trivial.
/// If you *are* implementing this for a data type, you must validate that the parameter type at `index` matches what you expect it to be.
/// You can usually do this by getting the raw parameter info and checking that the [`FMOD_DSP_PARAMETER_DATA_TYPE`] field matches what you expect.
///
/// There are hidden methods on [`Dsp`] that can help you write correct implementations for [`Sized`] types.
pub trait ReadableParameter: Sized {
    /// Get the parameter at `index`.
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self>;

    /// Get the parameter string at `index`.
    // FIXME Strings are a max of FMOD_DSP_GETPARAM_VALUESTR_LENGTH so we don't need to heap allocate them
    fn get_parameter_string(dsp: Dsp, index: c_int) -> Result<Utf8CString>;
}

/// Trait for types that can be written to DSP parameters.
///
/// You should either defer to [`Dsp::set_parameter`] or call [`FMOD_DSP_SetParameterData`].
///
/// # Data types
///
/// Implementing this trait for anything aside from data types is relatively trivial.
/// If you *are* implementing this for a data type, you must validate that the parameter type at `index` matches what you expect it to be.
/// You can usually do this by getting the raw parameter info and checking that the [`FMOD_DSP_PARAMETER_DATA_TYPE`] field matches what you expect.
///
/// There are hidden methods on [`Dsp`] that can help you write correct implementations for [`Sized`] types.
pub trait WritableParameter: Sized {
    /// Set the parameter at `index`.
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

            let string = Utf8CStr::from_utf8_until_nul(&bytes).unwrap().to_cstring();
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

            let string = Utf8CStr::from_utf8_until_nul(&bytes).unwrap().to_cstring();
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

            let string = Utf8CStr::from_utf8_until_nul(&bytes).unwrap().to_cstring();
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

/// Trait for types that can be turned into a *readable* parameter index.
pub trait ReadableParameterIndex<T> {
    /// What type of DSP this index is for.
    const TYPE: DspType;

    /// Convert `self` into a DSP index.
    fn into_index(self) -> c_int;
}

impl<T> ReadableParameterIndex<T> for c_int {
    const TYPE: DspType = DspType::Unknown;

    fn into_index(self) -> c_int {
        self
    }
}

/// Trait for types that can be turned into a *writable* parameter index.
pub trait WritableParameterIndex<T> {
    /// What type of DSP this index is for.
    const TYPE: DspType;

    /// Convert `self` into a DSP index.
    fn into_index(self) -> c_int;
}

impl<T> WritableParameterIndex<T> for c_int {
    const TYPE: DspType = DspType::Unknown;

    fn into_index(self) -> c_int {
        self
    }
}
