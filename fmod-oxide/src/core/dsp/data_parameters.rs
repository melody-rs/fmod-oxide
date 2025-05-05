use std::{
    ffi::{c_float, c_int},
    mem::MaybeUninit,
};

use fmod_sys::*;

use crate::{Attributes3D, Dsp, DspParameterDataType, ReadableDataParameter};

use super::parameters::WritableDataParameter;

fn parameter_is(dsp_parameter_desc: &FMOD_DSP_PARAMETER_DESC, kind: DspParameterDataType) -> bool {
    if dsp_parameter_desc.type_ != FMOD_DSP_PARAMETER_TYPE_DATA {
        return false;
    }
    unsafe { dsp_parameter_desc.__bindgen_anon_1.datadesc }.datatype == kind.into()
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct OverAlign {
    pub linear_gain: c_float,
    pub linear_gain_additive: c_float,
}

// Safety: we validate the data type matches what we expect.
unsafe impl ReadableDataParameter for OverAlign {
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        let desc = dsp.get_raw_parameter_info(index)?;
        if !parameter_is(&desc, DspParameterDataType::OverAlign) {
            return Err(Error::Fmod(FMOD_RESULT::FMOD_ERR_INVALID_PARAM));
        }
        let mut this = MaybeUninit::uninit();
        // Safety: we already validated that this is the right data type, so this is safe.
        unsafe { dsp.get_raw_parameter_data(&mut this, index)? };
        Ok(unsafe { this.assume_init() })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct DspAttributes3D {
    pub relative: Attributes3D,
    pub absolute: Attributes3D,
}

unsafe impl ReadableDataParameter for DspAttributes3D {
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        let desc = dsp.get_raw_parameter_info(index)?;
        if !parameter_is(&desc, DspParameterDataType::Attributes3D) {
            return Err(Error::Fmod(FMOD_RESULT::FMOD_ERR_INVALID_PARAM));
        }
        let mut this = MaybeUninit::uninit();
        // Safety: we already validated that this is the right data type, so this is safe.
        unsafe { dsp.get_raw_parameter_data(&mut this, index)? };
        Ok(unsafe { this.assume_init() })
    }
}

unsafe impl WritableDataParameter for DspAttributes3D {
    fn set_parameter(self, dsp: Dsp, index: c_int) -> Result<()> {
        let desc = dsp.get_raw_parameter_info(index)?;
        if !parameter_is(&desc, DspParameterDataType::Attributes3D) {
            return Err(Error::Fmod(FMOD_RESULT::FMOD_ERR_INVALID_PARAM));
        }
        // Safety: we already validated that this is the right data type, so this is safe.
        unsafe { dsp.set_raw_parameter_data(&self, index) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Sidechain {
    pub enable: bool,
}

unsafe impl ReadableDataParameter for Sidechain {
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        let desc = dsp.get_raw_parameter_info(index)?;
        if !parameter_is(&desc, DspParameterDataType::Attributes3D) {
            return Err(Error::Fmod(FMOD_RESULT::FMOD_ERR_INVALID_PARAM));
        }
        let mut raw = MaybeUninit::<FMOD_DSP_PARAMETER_SIDECHAIN>::uninit();
        // Safety: we already validated that this is the right data type, so this is safe.
        unsafe { dsp.get_raw_parameter_data(&mut raw, index)? };
        let raw = unsafe { raw.assume_init() };
        Ok(Self {
            enable: raw.sidechainenable.into(),
        })
    }
}

unsafe impl WritableDataParameter for Sidechain {
    fn set_parameter(self, dsp: Dsp, index: c_int) -> Result<()> {
        let desc = dsp.get_raw_parameter_info(index)?;
        if !parameter_is(&desc, DspParameterDataType::Attributes3D) {
            return Err(Error::Fmod(FMOD_RESULT::FMOD_ERR_INVALID_PARAM));
        }
        let raw = FMOD_DSP_PARAMETER_SIDECHAIN {
            sidechainenable: self.enable.into(),
        };
        // Safety: we already validated that this is the right data type, so this is safe.
        unsafe { dsp.set_raw_parameter_data(&raw, index) }
    }
}

// TODO FFT

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct DspAttributes3DMulti {
    // FIXME these should be slices?
    pub listener_count: c_int,
    pub relative: [Attributes3D; FMOD_MAX_LISTENERS as usize],
    pub weight: [c_float; FMOD_MAX_LISTENERS as usize],
    pub absolute: FMOD_3D_ATTRIBUTES,
}

unsafe impl ReadableDataParameter for DspAttributes3DMulti {
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        let desc = dsp.get_raw_parameter_info(index)?;
        if !parameter_is(&desc, DspParameterDataType::Attributes3DMulti) {
            return Err(Error::Fmod(FMOD_RESULT::FMOD_ERR_INVALID_PARAM));
        }
        let mut raw = MaybeUninit::uninit();
        // Safety: we already validated that this is the right data type, so this is safe.
        unsafe { dsp.get_raw_parameter_data(&mut raw, index)? };
        Ok(unsafe { raw.assume_init() })
    }
}

unsafe impl WritableDataParameter for DspAttributes3DMulti {
    fn set_parameter(self, dsp: Dsp, index: c_int) -> Result<()> {
        let desc = dsp.get_raw_parameter_info(index)?;
        if !parameter_is(&desc, DspParameterDataType::Attributes3DMulti) {
            return Err(Error::Fmod(FMOD_RESULT::FMOD_ERR_INVALID_PARAM));
        }
        // Safety: we already validated that this is the right data type, so this is safe.
        unsafe { dsp.set_raw_parameter_data(&self, index) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct AttenuationRange {
    pub min: c_float,
    pub max: c_float,
}

unsafe impl ReadableDataParameter for AttenuationRange {
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        let desc = dsp.get_raw_parameter_info(index)?;
        if !parameter_is(&desc, DspParameterDataType::AttenuationRange) {
            return Err(Error::Fmod(FMOD_RESULT::FMOD_ERR_INVALID_PARAM));
        }
        let mut raw = MaybeUninit::uninit();
        // Safety: we already validated that this is the right data type, so this is safe.
        unsafe { dsp.get_raw_parameter_data(&mut raw, index)? };
        Ok(unsafe { raw.assume_init() })
    }
}

#[cfg(fmod_2_3)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct DynamicResponse {
    pub channel_count: c_int,
    pub rms: [c_float; 32],
}

#[cfg(fmod_2_3)]
unsafe impl ReadableDataParameter for DynamicResponse {
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        let desc = dsp.get_raw_parameter_info(index)?;
        if !parameter_is(&desc, DspParameterDataType::DynamicResponse) {
            return Err(Error::Fmod(FMOD_RESULT::FMOD_ERR_INVALID_PARAM));
        }
        let mut raw = MaybeUninit::uninit();
        // Safety: we already validated that this is the right data type, so this is safe.
        unsafe { dsp.get_raw_parameter_data(&mut raw, index)? };
        Ok(unsafe { raw.assume_init() })
    }
}
