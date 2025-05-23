use std::{
    ffi::{c_float, c_int},
    mem::MaybeUninit,
};

use fmod_sys::*;

use crate::{Attributes3D, Dsp, DspParameterDataType, ReadableParameter, WritableParameter};
use crate::{Error, Result};

#[cfg(doc)]
use crate::{System, studio};

fn parameter_is(dsp_parameter_desc: &FMOD_DSP_PARAMETER_DESC, kind: DspParameterDataType) -> bool {
    if dsp_parameter_desc.type_ != FMOD_DSP_PARAMETER_TYPE_DATA {
        return false;
    }
    unsafe { dsp_parameter_desc.__bindgen_anon_1.datadesc }.datatype == kind.into()
}

/// Overall gain parameter data structure.
///
/// This parameter is read by the system to determine the effect's gain for voice virtualization.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct OverallGain {
    /// Overall linear gain of the effect on the direct signal path.
    pub linear_gain: c_float,
    /// Additive gain for parallel signal paths.
    pub linear_gain_additive: c_float,
}

// Safety: we validate the data type matches what we expect.
impl ReadableParameter for OverallGain {
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        let desc = dsp.get_raw_parameter_info(index)?;
        if !parameter_is(&desc, DspParameterDataType::OverAlign) {
            return Err(Error::InvalidParam);
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

/// 3D attributes data structure.
///
/// The [`studio::System`] sets this parameter automatically when an [`studio::EventInstance`] position changes.
/// However, if you are using the core [`System`] and not the [`studio::System`], you must set this DSP parameter explicitly.
///
/// Attributes must use a coordinate system with the positive Y axis being up and the positive X axis being right.
/// The FMOD Engine converts passed-in coordinates to left-handed for the plug-in if the system was initialized with the [`FMOD_INIT_3D_RIGHTHANDED`] flag.
///
/// When using a listener attenuation position,
/// the direction of the relative attributes will be relative to the listener position and the length will be the distance to the attenuation position.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct DspAttributes3D {
    /// Position of the sound relative to the listener.
    pub relative: Attributes3D,
    /// Position of the sound in world coordinates.
    pub absolute: Attributes3D,
}

impl ReadableParameter for DspAttributes3D {
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        let desc = dsp.get_raw_parameter_info(index)?;
        if !parameter_is(&desc, DspParameterDataType::Attributes3D) {
            return Err(Error::InvalidParam);
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
            return Err(Error::InvalidParam);
        }
        // Safety: we already validated that this is the right data type, so this is safe.
        unsafe { dsp.set_raw_parameter_data(&self, index) }
    }
}

/// Side chain parameter data structure.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Sidechain {
    /// Whether sidechains are enabled.
    pub enable: bool,
}

impl ReadableParameter for Sidechain {
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        let desc = dsp.get_raw_parameter_info(index)?;
        if !parameter_is(&desc, DspParameterDataType::Attributes3D) {
            return Err(Error::InvalidParam);
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
            return Err(Error::InvalidParam);
        }
        let raw = FMOD_DSP_PARAMETER_SIDECHAIN {
            sidechainenable: self.enable.into(),
        };
        // Safety: we already validated that this is the right data type, so this is safe.
        unsafe { dsp.set_raw_parameter_data(&raw, index) }
    }
}

/// FFT parameter data structure.
#[derive(Debug, Clone, PartialEq)]
pub struct Fft {
    channels: usize,
    spectrum_size: usize,
    data: Box<[c_float]>,
}

impl Fft {
    /// Number of channels in spectrum.
    pub fn channels(&self) -> usize {
        self.channels
    }

    /// Number of entries in this spectrum window.
    ///
    /// Divide this by the output rate to get the hz per entry.
    pub fn spectrum_size(&self) -> usize {
        self.spectrum_size
    }

    /// Channel spectrum data.
    ///
    /// Values inside the float buffer are typically between 0 and 1.0.
    /// Each top level array represents one PCM channel of data.
    ///
    /// Address data as `spectrum(channel)[bin]`. A bin is 1 fft window entry.
    ///
    /// Only read/display half of the buffer typically for analysis as the 2nd half is usually the same data reversed due to the nature of the way FFT works.
    pub fn spectrum(&self, channel: usize) -> &[c_float] {
        let offset = self.spectrum_size * channel;
        &self.data[offset..offset + self.spectrum_size]
    }

    /// Per channel spectrum arrays.
    pub fn data(&self) -> &[c_float] {
        &self.data
    }
}

// So glad this is read only because this would be AWFUL to implement writing for
impl ReadableParameter for Fft {
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        let desc = dsp.get_raw_parameter_info(index)?;
        if !parameter_is(&desc, DspParameterDataType::Attributes3D) {
            return Err(Error::InvalidParam);
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

/// 3D attributes data structure for multiple listeners.
///
/// The [`studio::System`] sets this parameter automatically when an [`studio::EventInstance`] position changes.
/// However, if you are using the core API's [`System`] and not the [`studio::System`],
/// you must set this DSP parameter explicitly.
///
/// Attributes must use a coordinate system with the positive Y axis being up and the positive X axis being right.
/// The FMOD Engine converts passed in coordinates to left-handed for the plug-in if the System was initialized with the [`FMOD_INIT_3D_RIGHTHANDED`] flag.
///
/// When using a listener attenuation position,
/// the direction of the relative attributes will be relative to the listener position and the length will be the distance to the attenuation position.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Attributes3DMulti {
    listener_count: c_int,
    relative: [Attributes3D; FMOD_MAX_LISTENERS as usize],
    weight: [c_float; FMOD_MAX_LISTENERS as usize],
    /// Position of the sound in world coordinates.
    pub absolute: Attributes3D,
}

impl Attributes3DMulti {
    /// Create a new [`Attributes3DMulti`].
    ///
    /// Only values from <code>data[..[FMOD_MAX_LISTENERS]]</code> will be read
    pub fn new(data: &[(Attributes3D, c_float)], absolute: Attributes3D) -> Self {
        let relative = std::array::from_fn(|i| data.get(i).map(|d| d.0).unwrap_or_default());
        let weight = std::array::from_fn(|i| data.get(i).map(|d| d.1).unwrap_or_default());
        Self {
            listener_count: data.len() as _,
            relative,
            weight,
            absolute,
        }
    }

    /// Position of the sound relative to the listeners.
    pub fn relative(&self) -> &[Attributes3D] {
        &self.relative[..self.listener_count as _]
    }

    /// Position of the sound relative to the listeners.
    pub fn relative_mut(&mut self) -> &mut [Attributes3D] {
        &mut self.relative[..self.listener_count as _]
    }

    /// Weighting of the listeners where 0 means listener has no contribution and 1 means full contribution.
    pub fn weight(&self) -> &[c_float] {
        &self.weight[..self.listener_count as _]
    }

    /// Weighting of the listeners where 0 means listener has no contribution and 1 means full contribution.
    pub fn weight_mut(&mut self) -> &mut [c_float] {
        &mut self.weight[..self.listener_count as _]
    }

    /// Number of listeners.
    pub fn listener_count(&self) -> usize {
        self.listener_count as _
    }
}

impl ReadableParameter for Attributes3DMulti {
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        let desc = dsp.get_raw_parameter_info(index)?;
        if !parameter_is(&desc, DspParameterDataType::Attributes3DMulti) {
            return Err(Error::InvalidParam);
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
            return Err(Error::InvalidParam);
        }
        // Safety: we already validated that this is the right data type, so this is safe.
        unsafe { dsp.set_raw_parameter_data(&self, index) }
    }
}
/// Attenuation range parameter data structure.
///
/// The [`studio::System`] will set this parameter automatically if an [`studio::EventInstance`] min or max distance changes.
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct AttenuationRange {
    /// Minimum distance for attenuation.
    pub min: c_float,
    /// Maximum distance for attenuation.
    pub max: c_float,
}

impl ReadableParameter for AttenuationRange {
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        let desc = dsp.get_raw_parameter_info(index)?;
        if !parameter_is(&desc, DspParameterDataType::AttenuationRange) {
            return Err(Error::InvalidParam);
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

/// Dynamic response data structure.
#[cfg(fmod_2_3)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct DynamicResponse {
    /// The number of channels recorded in the rms array.
    pub channel_count: c_int,
    /// The RMS (Root Mean Square) averaged gain factor applied per channel for the last processed block of audio.
    pub rms: [c_float; 32],
}

#[cfg(fmod_2_3)]
impl ReadableParameter for DynamicResponse {
    fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
        let desc = dsp.get_raw_parameter_info(index)?;
        if !parameter_is(&desc, DspParameterDataType::DynamicResponse) {
            return Err(Error::InvalidParam);
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
