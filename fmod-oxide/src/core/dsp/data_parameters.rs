use std::{
    ffi::{c_float, c_int},
    mem::MaybeUninit,
};

use fmod_sys::*;

use crate::{Attributes3D, Dsp, DspParameterDataType, ReadableParameter, WritableParameter};

fn parameter_is(dsp_parameter_desc: &FMOD_DSP_PARAMETER_DESC, kind: DspParameterDataType) -> bool {
    if dsp_parameter_desc.type_ != FMOD_DSP_PARAMETER_TYPE_DATA {
        return false;
    }
    unsafe { dsp_parameter_desc.__bindgen_anon_1.datadesc }.datatype == kind.into()
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct OverallGain {
    pub linear_gain: c_float,
    pub linear_gain_additive: c_float,
}

// Safety: we validate the data type matches what we expect.
impl ReadableParameter for OverallGain {
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

    fn get_parameter_string(dsp: Dsp, index: c_int) -> Result<lanyard::Utf8CString> {
        dsp.get_data_parameter_string(index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct DspAttributes3D {
    pub relative: Attributes3D,
    pub absolute: Attributes3D,
}

impl ReadableParameter for DspAttributes3D {
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

    fn get_parameter_string(dsp: Dsp, index: c_int) -> Result<lanyard::Utf8CString> {
        dsp.get_data_parameter_string(index)
    }
}

impl WritableParameter for DspAttributes3D {
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

impl ReadableParameter for Sidechain {
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

    fn get_parameter_string(dsp: Dsp, index: c_int) -> Result<lanyard::Utf8CString> {
        dsp.get_data_parameter_string(index)
    }
}

impl WritableParameter for Sidechain {
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

#[derive(Debug, Clone, PartialEq)]
pub struct Fft {
    channels: usize,
    spectrum_size: usize,
    data: Box<[c_float]>,
}

impl Fft {
    pub fn channels(&self) -> usize {
        self.channels
    }

    pub fn spectrum_size(&self) -> usize {
        self.spectrum_size
    }

    pub fn spectrum(&self, channel: usize) -> &[c_float] {
        let offset = self.spectrum_size * channel;
        &self.data[offset..offset + self.spectrum_size]
    }

    pub fn data(&self) -> &[c_float] {
        &self.data
    }
}

// So glad this is read only because this would be AWFUL to implement writing for
impl ReadableParameter for Fft {
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        let desc = dsp.get_raw_parameter_info(index)?;
        if !parameter_is(&desc, DspParameterDataType::Attributes3D) {
            return Err(Error::Fmod(FMOD_RESULT::FMOD_ERR_INVALID_PARAM));
        }
        let mut raw = MaybeUninit::<FMOD_DSP_PARAMETER_FFT>::uninit();
        // Safety: we already validated that this is the right data type, so this is safe.
        unsafe { dsp.get_raw_parameter_data(&mut raw, index)? };
        let raw = unsafe { raw.assume_init() };

        let mut data = Vec::with_capacity(raw.numchannels as _);
        for i in 0..raw.numchannels as _ {
            let ptr = raw.spectrum[i];
            let slice = unsafe { std::slice::from_raw_parts(ptr, raw.length as _) };
            data.extend_from_slice(slice);
        }
        Ok(Self {
            channels: raw.numchannels as _,
            spectrum_size: raw.length as _,
            data: data.into_boxed_slice(),
        })
    }

    fn get_parameter_string(dsp: Dsp, index: c_int) -> Result<lanyard::Utf8CString> {
        dsp.get_data_parameter_string(index)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Attributes3DMulti {
    listener_count: c_int,
    relative: [Attributes3D; FMOD_MAX_LISTENERS as usize],
    weight: [c_float; FMOD_MAX_LISTENERS as usize],
    pub absolute: FMOD_3D_ATTRIBUTES,
}

impl Attributes3DMulti {
    pub fn new(data: &[(Attributes3D, c_float)], absolute: FMOD_3D_ATTRIBUTES) -> Self {
        let relative = std::array::from_fn(|i| data.get(i).map(|d| d.0).unwrap_or_default());
        let weight = std::array::from_fn(|i| data.get(i).map(|d| d.1).unwrap_or_default());
        Self {
            listener_count: data.len() as _,
            relative,
            weight,
            absolute,
        }
    }

    pub fn relative(&self) -> &[Attributes3D] {
        &self.relative[..self.listener_count as _]
    }

    pub fn relative_mut(&mut self) -> &mut [Attributes3D] {
        &mut self.relative[..self.listener_count as _]
    }

    pub fn weight(&self) -> &[c_float] {
        &self.weight[..self.listener_count as _]
    }

    pub fn weight_mut(&mut self) -> &mut [c_float] {
        &mut self.weight[..self.listener_count as _]
    }

    pub fn listener_count(&self) -> usize {
        self.listener_count as _
    }
}

impl ReadableParameter for Attributes3DMulti {
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

    fn get_parameter_string(dsp: Dsp, index: c_int) -> Result<lanyard::Utf8CString> {
        dsp.get_data_parameter_string(index)
    }
}

impl WritableParameter for Attributes3DMulti {
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

impl ReadableParameter for AttenuationRange {
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

    fn get_parameter_string(dsp: Dsp, index: c_int) -> Result<lanyard::Utf8CString> {
        dsp.get_data_parameter_string(index)
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
impl ReadableParameter for DynamicResponse {
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

    fn get_parameter_string(dsp: Dsp, index: c_int) -> Result<lanyard::Utf8CString> {
        dsp.get_data_parameter_string(index)
    }
}
