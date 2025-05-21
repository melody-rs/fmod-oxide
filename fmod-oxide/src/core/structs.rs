// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    ffi::{c_float, c_int, c_short, c_uchar, c_uint, c_ushort},
    mem::MaybeUninit,
};

use crate::{FmodResultExt, Result};
use fmod_sys::*;
use lanyard::{Utf8CStr, Utf8CString};

use super::{FloatMappingType, Resampler, Speaker};
use crate::{DspParameterDataType, TagType, string_from_utf16_be, string_from_utf16_le};

/// Structure describing a globally unique identifier.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
// force this type to have the exact same layout as FMOD_STUDIO_PARAMETER_ID so we can safely transmute between them.
#[repr(C)]
pub struct Guid {
    /// Specifies the first 8 hexadecimal digits of the GUID.
    pub data_1: c_uint,
    /// Specifies the first group of 4 hexadecimal digits.
    pub data_2: c_ushort,
    /// Specifies the second group of 4 hexadecimal digits.
    pub data_3: c_ushort,
    /// Specifies the second group of 4 hexadecimal digits.
    pub data_4: [c_uchar; 8],
}

impl Guid {
    /// Parse a GUID from a string.
    #[cfg(feature = "studio")]
    pub fn parse(string: &Utf8CStr) -> Result<Self> {
        let mut guid = MaybeUninit::uninit();
        unsafe {
            FMOD_Studio_ParseID(string.as_ptr(), guid.as_mut_ptr()).to_result()?;
            Ok(guid.assume_init().into())
        }
    }
}

impl From<FMOD_GUID> for Guid {
    fn from(value: FMOD_GUID) -> Self {
        Guid {
            data_1: value.Data1,
            data_2: value.Data2,
            data_3: value.Data3,
            data_4: value.Data4,
        }
    }
}

impl From<Guid> for FMOD_GUID {
    fn from(value: Guid) -> Self {
        FMOD_GUID {
            Data1: value.data_1,
            Data2: value.data_2,
            Data3: value.data_3,
            Data4: value.data_4,
        }
    }
}

impl std::fmt::Display for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Guid {
            data_1,
            data_2,
            data_3,
            data_4,
        } = self;

        f.write_str("{")?;
        f.write_fmt(format_args!("{data_1:0>8x}-{data_2:0>4x}-{data_3:0>4x}-"))?;
        f.write_fmt(format_args!("{:0>2x}{:0>2x}-", data_4[0], data_4[1]))?;
        for b in &data_4[2..] {
            f.write_fmt(format_args!("{b:0>2x}"))?;
        }
        f.write_str("}")
    }
}

/// Structure describing a point in 3D space.
///
/// FMOD uses a left handed coordinate system by default.
///
/// To use a right handed coordinate system specify [`FMOD_INIT_3D_RIGHTHANDED`] from [`FMOD_INITFLAGS`] in [`System::init`].
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
#[repr(C)]
pub struct Vector {
    /// X coordinate in 3D space.
    pub x: c_float,
    /// Y coordinate in 3D space.
    pub y: c_float,
    /// Z coordinate in 3D space.
    pub z: c_float,
}

impl From<Vector> for FMOD_VECTOR {
    fn from(value: Vector) -> Self {
        FMOD_VECTOR {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<FMOD_VECTOR> for Vector {
    fn from(value: FMOD_VECTOR) -> Self {
        Vector {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

/// Structure describing a position, velocity and orientation.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
#[repr(C)]
pub struct Attributes3D {
    /// Position in world space used for panning and attenuation.
    pub position: Vector,
    /// Velocity in world space used for doppler.
    pub velocity: Vector,
    /// Forwards orientation, must be of unit length (1.0) and perpendicular to up.
    pub forward: Vector,
    /// Upwards orientation, must be of unit length (1.0) and perpendicular to forward.
    pub up: Vector,
}

impl From<FMOD_3D_ATTRIBUTES> for Attributes3D {
    fn from(value: FMOD_3D_ATTRIBUTES) -> Self {
        Attributes3D {
            position: value.position.into(),
            velocity: value.velocity.into(),
            forward: value.forward.into(),
            up: value.up.into(),
        }
    }
}

impl From<Attributes3D> for FMOD_3D_ATTRIBUTES {
    fn from(value: Attributes3D) -> Self {
        FMOD_3D_ATTRIBUTES {
            position: value.position.into(),
            velocity: value.velocity.into(),
            forward: value.forward.into(),
            up: value.up.into(),
        }
    }
}

/// Performance information for Core API functionality.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct CpuUsage {
    /// DSP mixing engine CPU usage.
    ///
    /// Percentage of [`FMOD_THREAD_TYPE_MIXER`], or main thread if [`FMOD_INIT_MIX_FROM_UPDATE`] flag is used with [`System::init`].
    pub dsp: c_float,
    /// Streaming engine CPU usage.
    ///
    /// Percentage of [`FMOD_THREAD_TYPE_STREAM`], or main thread if [`FMOD_INIT_STREAM_FROM_UPDATE`] flag is used with [`System::init`].
    pub stream: c_float,
    /// Geometry engine CPU usage.
    ///
    /// Percentage of [`FMOD_THREAD_TYPE_GEOMETRY`].
    pub geometry: c_float,
    /// [`System::update`] CPU usage. Percentage of main thread.
    pub update: c_float,
    /// Convolution reverb processing thread #1 CPU usage.
    ///
    /// Percentage of [`FMOD_THREAD_TYPE_CONVOLUTION1`].
    pub convolution_1: c_float,
    /// Convolution reverb processing thread #2 CPU usage.
    ///
    /// Percentage of [`FMOD_THREAD_TYPE_CONVOLUTION2`].
    pub convolution_2: c_float,
}

impl From<FMOD_CPU_USAGE> for CpuUsage {
    fn from(value: FMOD_CPU_USAGE) -> Self {
        CpuUsage {
            dsp: value.dsp,
            stream: value.stream,
            geometry: value.geometry,
            update: value.update,
            convolution_1: value.convolution1,
            convolution_2: value.convolution2,
        }
    }
}

impl From<CpuUsage> for FMOD_CPU_USAGE {
    fn from(value: CpuUsage) -> Self {
        FMOD_CPU_USAGE {
            dsp: value.dsp,
            stream: value.stream,
            geometry: value.geometry,
            update: value.update,
            convolution1: value.convolution_1,
            convolution2: value.convolution_2,
        }
    }
}

/// Structure defining a reverb environment.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
#[repr(C)]
pub struct ReverbProperties {
    /// Reverberation decay time.
    pub decay_time: c_float,
    /// Initial reflection delay time.
    pub early_delay: c_float,
    /// Late reverberation delay time relative to initial reflection.
    pub late_delay: c_float,
    /// Reference high frequency.
    pub hf_reference: c_float,
    /// High-frequency to mid-frequency decay time ratio.
    pub hf_decay_ratio: c_float,
    /// Value that controls the echo density in the late reverberation decay.
    pub diffusion: c_float,
    /// Value that controls the modal density in the late reverberation decay.
    pub density: c_float,
    /// Reference low frequency
    pub low_shelf_frequency: c_float,
    /// Relative room effect level at low frequencies.
    pub low_shelf_gain: c_float,
    /// Relative room effect level at high frequencies.
    pub high_cut: c_float,
    /// Early reflections level relative to room effect.
    pub early_late_mix: c_float,
    /// Room effect level at mid frequencies.
    pub wet_level: c_float,
}

impl From<FMOD_REVERB_PROPERTIES> for ReverbProperties {
    fn from(value: FMOD_REVERB_PROPERTIES) -> Self {
        ReverbProperties {
            decay_time: value.DecayTime,
            early_delay: value.EarlyDelay,
            late_delay: value.LateDelay,
            hf_reference: value.HFReference,
            hf_decay_ratio: value.HFDecayRatio,
            diffusion: value.Diffusion,
            density: value.Density,
            low_shelf_frequency: value.LowShelfFrequency,
            low_shelf_gain: value.LowShelfGain,
            high_cut: value.HighCut,
            early_late_mix: value.EarlyLateMix,
            wet_level: value.WetLevel,
        }
    }
}

impl From<ReverbProperties> for FMOD_REVERB_PROPERTIES {
    fn from(value: ReverbProperties) -> Self {
        FMOD_REVERB_PROPERTIES {
            DecayTime: value.decay_time,
            EarlyDelay: value.early_delay,
            LateDelay: value.late_delay,
            HFReference: value.hf_reference,
            HFDecayRatio: value.hf_decay_ratio,
            Diffusion: value.diffusion,
            Density: value.density,
            LowShelfFrequency: value.low_shelf_frequency,
            LowShelfGain: value.low_shelf_gain,
            HighCut: value.high_cut,
            EarlyLateMix: value.early_late_mix,
            WetLevel: value.wet_level,
        }
    }
}

/// Base structure for DSP parameter descriptions.
#[derive(Debug)]
pub struct DspParameterDescription {
    /// Parameter type.
    pub kind: DspParameterType,
    /// Parameter name.
    pub name: Utf8CString,
    /// Parameter label.
    pub label: Utf8CString,
    /// Parameter description.
    pub description: Utf8CString,
}

/// DSP parameter types.
#[derive(Clone, Debug, PartialEq)]
pub enum DspParameterType {
    /// Float parameter description.
    Float {
        /// Minimum value.
        min: f32,
        /// Maximum value.
        max: f32,
        /// Default value.
        default: f32,
        /// How the values are distributed across dials and automation curves.
        mapping: FloatMapping,
    },
    /// Integer parameter description.
    Int {
        /// Minimum value.
        min: i32,
        /// Maximum value.
        max: i32,
        /// Default value.
        default: i32,
        /// Whether the last value represents infinity.
        goes_to_infinity: bool,
        /// Names for each value (UTF-8 string).
        ///
        /// There should be as many strings as there are possible values (max - min + 1).
        names: Option<Vec<Utf8CString>>,
    },
    /// Boolean parameter description.
    Bool {
        /// Default parameter value.
        default: bool,
        /// Names for false and true, respectively (UTF-8 string).
        names: Option<[Utf8CString; 2]>,
    },
    /// Data parameter description.
    Data {
        /// Type of data.
        data_type: DspParameterDataType,
    },
}

/// Structure to define a mapping for a DSP unit's float parameter.
#[derive(Clone, Debug, PartialEq)]
pub struct FloatMapping {
    /// Float mapping type.
    pub kind: FloatMappingType,
    /// Piecewise linear mapping type.
    pub piecewise_linear_mapping: Option<PiecewiseLinearMapping>,
}

/// Structure to define a piecewise linear mapping.
#[derive(Clone, Debug, PartialEq)]
pub struct PiecewiseLinearMapping {
    /// Values in the parameter's units for each point .
    pub point_param_values: Vec<c_float>,
    /// Positions along the control's scale (e.g. dial angle) corresponding to each parameter value.
    ///
    /// The range of this scale is arbitrary and all positions will be relative to the minimum and maximum values
    /// (e.g. [0,1,3] is equivalent to [1,2,4] and [2,4,8]).
    ///
    /// If this array is `None`, `point_param_values` will be distributed with equal spacing.
    pub point_positions: Option<Vec<c_float>>,
}

impl FloatMapping {
    unsafe fn from_ffi(value: FMOD_DSP_PARAMETER_FLOAT_MAPPING) -> Self {
        let kind = value.type_.try_into().unwrap();

        let piecewise_linear_mapping = if kind == FloatMappingType::PiecewiceLinear {
            let point_param_values = unsafe {
                std::slice::from_raw_parts(
                    value.piecewiselinearmapping.pointparamvalues,
                    value.piecewiselinearmapping.numpoints as _,
                )
                .to_vec()
            };
            let point_positions = if value.piecewiselinearmapping.pointpositions.is_null() {
                None
            } else {
                Some(unsafe {
                    std::slice::from_raw_parts(
                        value.piecewiselinearmapping.pointpositions,
                        value.piecewiselinearmapping.numpoints as _,
                    )
                    .to_vec()
                })
            };
            Some(PiecewiseLinearMapping {
                point_param_values,
                point_positions,
            })
        } else {
            None
        };

        Self {
            kind,
            piecewise_linear_mapping,
        }
    }
}

impl DspParameterDescription {
    /// Create a safe [`DspParameterDescription`] struct from the FFI equivalent.
    ///
    /// # Safety
    ///
    /// [`FMOD_DSP_PARAMETER_DESC::type_`] must match the union value.
    ///
    /// The strings [`FMOD_DSP_PARAMETER_DESC`] must be a null-terminated and must be valid for reads of bytes up to and including the nul terminator.
    ///
    /// See [`Utf8CStr::from_ptr_unchecked`] for more information.
    ///
    /// # Panics
    ///
    /// This function will panic if the description type is not valid.
    pub unsafe fn from_ffi(value: FMOD_DSP_PARAMETER_DESC) -> Self {
        // FIXME these array accesses are safe and could be done in a safer way
        let name = unsafe { Utf8CStr::from_ptr_unchecked(value.name.as_ptr()).to_cstring() };
        let label = unsafe { Utf8CStr::from_ptr_unchecked(value.label.as_ptr()).to_cstring() };
        let description = unsafe { Utf8CStr::from_ptr_unchecked(value.description).to_cstring() };
        let kind = match value.type_ {
            FMOD_DSP_PARAMETER_TYPE_FLOAT => {
                let floatdesc = unsafe { value.__bindgen_anon_1.floatdesc };
                let mapping = unsafe { FloatMapping::from_ffi(floatdesc.mapping) };

                DspParameterType::Float {
                    min: floatdesc.min,
                    max: floatdesc.max,
                    default: floatdesc.defaultval,
                    mapping,
                }
            }
            FMOD_DSP_PARAMETER_TYPE_INT => {
                let intdesc = unsafe { value.__bindgen_anon_1.intdesc };
                let names = if intdesc.valuenames.is_null() {
                    None
                } else {
                    let pointers = unsafe {
                        std::slice::from_raw_parts(
                            intdesc.valuenames,
                            intdesc.max as usize - intdesc.min as usize + 1,
                        )
                    };
                    Some(
                        pointers
                            .iter()
                            .map(|p| unsafe { Utf8CStr::from_ptr_unchecked(*p).to_cstring() })
                            .collect(),
                    )
                };

                DspParameterType::Int {
                    min: intdesc.min,
                    max: intdesc.max,
                    default: intdesc.defaultval,
                    goes_to_infinity: intdesc.goestoinf.into(),
                    names,
                }
            }
            FMOD_DSP_PARAMETER_TYPE_BOOL => {
                let booldesc = unsafe { value.__bindgen_anon_1.booldesc };
                let names = if booldesc.valuenames.is_null() {
                    None
                } else {
                    let [p1, p2] =
                        unsafe { *std::ptr::from_ref(&booldesc.valuenames).cast::<[_; 2]>() };
                    Some([
                        unsafe { Utf8CStr::from_ptr_unchecked(p1).to_cstring() },
                        unsafe { Utf8CStr::from_ptr_unchecked(p2).to_cstring() },
                    ])
                };

                DspParameterType::Bool {
                    default: booldesc.defaultval.into(),
                    names,
                }
            }
            FMOD_DSP_PARAMETER_TYPE_DATA => {
                let datadesc = unsafe { value.__bindgen_anon_1.datadesc };
                DspParameterType::Data {
                    data_type: datadesc.datatype.into(),
                }
            }
            _ => panic!("invalid parameter description type"), // FIXME panic
        };
        Self {
            kind,
            name,
            label,
            description,
        }
    }

    // No FFI conversion is provided because we don't support writing dsps in rust yet
}

/// DSP metering info.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DspMeteringInfo {
    /// Number of samples considered for this metering info.
    pub sample_count: c_int,
    /// Peak level per channel.
    pub peak_level: [c_float; 32],
    /// Rms level per channel.
    pub rms_level: [c_float; 32],
    /// Number of channels.
    pub channel_count: c_short,
}

impl From<FMOD_DSP_METERING_INFO> for DspMeteringInfo {
    fn from(value: FMOD_DSP_METERING_INFO) -> Self {
        Self {
            sample_count: value.numsamples,
            peak_level: value.peaklevel,
            rms_level: value.rmslevel,
            channel_count: value.numchannels,
        }
    }
}

impl From<DspMeteringInfo> for FMOD_DSP_METERING_INFO {
    fn from(value: DspMeteringInfo) -> Self {
        FMOD_DSP_METERING_INFO {
            numsamples: value.sample_count,
            peaklevel: value.peak_level,
            rmslevel: value.rms_level,
            numchannels: value.channel_count,
        }
    }
}

/// Tag data / metadata description.
#[derive(Debug)]
pub struct Tag {
    /// Tag type.
    pub kind: TagType,
    /// Name.
    pub name: Utf8CString,
    /// Tag data type.
    pub data: TagData,
    /// True if this tag has been updated since last being accessed with [`Sound::getTag`]
    pub updated: bool,
}

/// List of tag data / metadata types.
#[derive(Debug)]
// FIXME: these strings are most likely null-terminated
pub enum TagData {
    /// Raw binary data.
    Binary(Vec<u8>),
    /// Integer.
    Integer(i64),
    /// IEEE floating point number.
    Float(f64),
    /// 8bit ASCII char string.
    String(String),
    /// 8 bit UTF string.
    Utf8String(String),
    /// 16bit UTF string Big endian byte order.
    Utf16StringBE(String),
    /// 16bit UTF string. Assume little endian byte order.
    Utf16String(String),
}

impl Tag {
    /// Create a safe [`Tag`] struct from the FFI equivalent.
    ///
    /// # Safety
    ///
    /// The string [`FMOD_TAG::name`] must be a null-terminated and must be valid for reads of bytes up to and including the nul terminator.
    ///
    /// This function will read into arbitrary memory! Because of this the tag data type must match the data type of the data pointer.
    ///
    /// # Panics
    ///
    /// This function will panic if `value` is not valid (Invalid type, wrong data length, etc)
    #[allow(clippy::cast_lossless)]
    pub unsafe fn from_ffi(value: FMOD_TAG) -> Self {
        let kind = value.type_.try_into().unwrap();
        let name = unsafe { Utf8CStr::from_ptr_unchecked(value.name).to_cstring() };
        let updated = value.updated.into();
        let data = unsafe {
            // awful union-esquqe code
            match value.datatype {
                FMOD_TAGDATATYPE_BINARY => {
                    let slice =
                        std::slice::from_raw_parts(value.data as *const u8, value.datalen as usize);
                    TagData::Binary(slice.to_vec())
                }
                FMOD_TAGDATATYPE_INT => match value.datalen {
                    1 => TagData::Integer(*value.data.cast::<i8>() as i64),
                    2 => TagData::Integer(*value.data.cast::<i16>() as i64),
                    4 => TagData::Integer(*value.data.cast::<i32>() as i64),
                    8 => TagData::Integer(*value.data.cast::<i64>()),
                    _ => panic!("unrecognized integer data len"),
                },
                FMOD_TAGDATATYPE_FLOAT => match value.datalen {
                    4 => TagData::Float(*value.data.cast::<f32>() as f64),
                    8 => TagData::Float(*value.data.cast::<f64>()),
                    _ => panic!("unrecognized float data len"),
                },
                FMOD_TAGDATATYPE_STRING => {
                    let ascii =
                        std::slice::from_raw_parts(value.data.cast(), value.datalen as usize);
                    let string = String::from_utf8_lossy(ascii).into_owned();
                    TagData::String(string)
                }
                FMOD_TAGDATATYPE_STRING_UTF8 => {
                    let utf8 =
                        std::slice::from_raw_parts(value.data.cast(), value.datalen as usize);
                    let string = String::from_utf8_lossy(utf8).into_owned();
                    TagData::Utf8String(string)
                }
                // depending on the architecture rust will optimize this to a no-op
                // we still need to do this to ensure the correct endianness
                // ideally we could use String::from_utf16_be_lossy but that is nightly only and the tracking issue has basically no activity
                FMOD_TAGDATATYPE_STRING_UTF16 => {
                    let slice =
                        std::slice::from_raw_parts(value.data.cast(), value.datalen as usize);
                    let string = string_from_utf16_le(slice);
                    TagData::Utf16String(string)
                }
                FMOD_TAGDATATYPE_STRING_UTF16BE => {
                    let slice =
                        std::slice::from_raw_parts(value.data.cast(), value.datalen as usize);
                    let string = string_from_utf16_be(slice);
                    TagData::Utf16StringBE(string)
                }
                _ => panic!("unrecognized tag data type"), // FIXME panic
            }
        };
        Tag {
            kind,
            name,
            data,
            updated,
        }
    }
}

/// Advanced configuration settings.
///
/// Structure to allow configuration of lesser used system level settings.
/// These tweaks generally allow the user to set resource limits and customize settings to better fit their application.
#[derive(Debug, Default)]
pub struct AdvancedSettings {
    /// Maximum MPEG Sounds created as [`FMOD_CREATECOMPRESSEDSAMPLE`].
    pub max_mpeg_codecs: c_int,
    /// Maximum IMA-ADPCM Sounds created as [`FMOD_CREATECOMPRESSEDSAMPLE`].
    pub max_adpcm_codecs: c_int,
    /// Maximum XMA Sounds created as [`FMOD_CREATECOMPRESSEDSAMPLE`].
    pub max_xma_codecs: c_int,
    /// Maximum Vorbis Sounds created as [`FMOD_CREATECOMPRESSEDSAMPLE`].
    pub max_vorbis_codecs: c_int,
    /// Maximum AT9 Sounds created as [`FMOD_CREATECOMPRESSEDSAMPLE`].
    pub max_at9_codecs: c_int,
    /// Maximum FADPCM Sounds created as [`FMOD_CREATECOMPRESSEDSAMPLE`].
    pub max_fadpcm_codecs: c_int,
    /// Maximum Opus Sounds created as [`FMOD_CREATECOMPRESSEDSAMPLE`].
    pub max_opus_codecs: c_int,
    #[cfg(fmod_lt_2_3)]
    pub max_pcm_codecs: c_int,

    // The docs mention something about this "not being valid before System::init"
    // No idea what that means. I don't think it's anything we need to worry about?
    // This is also not used when calling `SetAdvancedSettings` so we don't need to worry about asio_speaker_list matching the same length.
    // I *think*.
    // Should this be an enum?
    /// Read only list of strings representing ASIO channel names (UTF-8 string).
    pub asio_channel_list: Option<Vec<Utf8CString>>,
    /// List of speakers that represent each ASIO channel used for remapping.
    ///
    /// Use [`FMOD_SPEAKER_NONE`] to indicate no output for a given speaker.
    pub asio_speaker_list: Option<Vec<Speaker>>, // FIXME: validate this is copied
    /// For use with [`FMOD_INIT_VOL0_BECOMES_VIRTUAL`],
    ///
    /// [`Channel`]s with audibility below this will become virtual.
    ///
    /// See the Virtual Voices guide for more information.
    pub vol0_virtual_vol: c_float,
    /// For use with Streams, the default size of the double buffer.
    pub default_decode_buffer_size: c_uint,
    /// For use with [`FMOD_INIT_PROFILE_ENABLE`],
    /// specify the port to listen on for connections by FMOD Studio or FMOD Profiler.
    pub profile_port: c_ushort,
    /// For use with [`Geometry`],
    /// the maximum time it takes for a [`Channel`] to fade to the new volume level when its occlusion changes.
    pub geometry_max_fade_time: c_uint,
    /// For use with [`FMOD_INIT_CHANNEL_DISTANCEFILTER`],
    /// the default center frequency for the distance filter.
    pub distance_filter_center_freq: c_float,
    /// For use with [`Reverb3D`], selects which global reverb instance to use.
    pub reverb_3d_instance: c_int,
    /// Number of intermediate mixing buffers in the DSP buffer pool.
    /// Each buffer in bytes is `dsp_buffer_pool_size` (See [`System::getDSPBufferSize`]) * sizeof(float) * output mode speaker count.
    ///
    /// ie 7.1 @ 1024 DSP block size = 1024 * 4 * 8 = 32KB.
    pub dsp_buffer_pool_size: c_int,
    /// Resampling method used by [`Channel`]s.
    pub resampler_method: Resampler,
    /// Seed value to initialize the internal random number generator.
    pub random_seed: c_uint,
    /// Maximum number of CPU threads to use for [`FMOD_DSP_TYPE_CONVOLUTIONREVERB`] effect.
    ///
    /// 1 = effect is entirely processed inside the [`FMOD_THREAD_TYPE_MIXER`] thread.
    ///
    /// 2 and 3 offloads different parts of the convolution processing into different threads
    /// ([`FMOD_THREAD_TYPE_CONVOLUTION1`] and [`FMOD_THREAD_TYPE_CONVOLUTION2`]) to increase throughput.
    pub max_convolution_threads: c_int,
    /// Maximum Spatial Objects that can be reserved per FMOD system.
    ///
    /// [`FMOD_OUTPUTTYPE_AUDIO3D`] is a special case where multiple FMOD systems are not allowed.
    ///
    /// See the Object based approach section of the Spatial Audio white paper.
    /// - A value of -1 means no Spatial Objects will be reserved.
    /// - A value of 0 means all available Spatial Objects will be reserved.
    /// - Any other value means it will reserve that many Spatial Objects.
    pub max_spatial_objects: c_int,
}

impl From<&AdvancedSettings> for FMOD_ADVANCEDSETTINGS {
    fn from(value: &AdvancedSettings) -> Self {
        let speaker_count = value.asio_speaker_list.as_ref().map_or(0, Vec::len);
        let speaker_ptr: *const Speaker = value
            .asio_speaker_list
            .as_ref()
            .map_or(std::ptr::null_mut(), Vec::as_ptr);

        Self {
            cbSize: size_of::<FMOD_ADVANCEDSETTINGS>() as c_int,
            maxMPEGCodecs: value.max_mpeg_codecs,
            maxADPCMCodecs: value.max_adpcm_codecs,
            maxXMACodecs: value.max_xma_codecs,
            maxVorbisCodecs: value.max_vorbis_codecs,
            maxAT9Codecs: value.max_at9_codecs,
            maxFADPCMCodecs: value.max_fadpcm_codecs,
            maxOpusCodecs: value.max_opus_codecs,
            #[cfg(fmod_lt_2_3)]
            maxPCMCodecs: value.max_pcm_codecs,
            ASIONumChannels: speaker_count as i32,
            ASIOChannelList: std::ptr::null_mut(),
            // Speaker has the same repr() as i32
            // So this SHOULD be ok
            ASIOSpeakerList: speaker_ptr.cast_mut().cast(),
            vol0virtualvol: value.vol0_virtual_vol,
            defaultDecodeBufferSize: value.default_decode_buffer_size,
            profilePort: value.profile_port,
            geometryMaxFadeTime: value.geometry_max_fade_time,
            distanceFilterCenterFreq: value.distance_filter_center_freq,
            reverb3Dinstance: value.reverb_3d_instance,
            DSPBufferPoolSize: value.dsp_buffer_pool_size,
            resamplerMethod: value.resampler_method.into(),
            randomSeed: value.random_seed,
            maxConvolutionThreads: value.max_convolution_threads,
            maxSpatialObjects: value.max_spatial_objects,
        }
    }
}

impl AdvancedSettings {
    /// Due to how [`FMOD_ADVANCEDSETTINGS`] interacts with `FMOD_System_GetAdvancedSettings` this won't read `ASIOSpeakerList`.
    /// Usually `ASIOSpeakerList` won't be filled out. If you're 100% certain that's not the case, you will have to convert it yourself.
    ///
    /// ```ignore
    /// let slice = unsafe { std::slice::from_raw_parts(value.ASIOSpeakerList, value.ASIONumChannels) };
    /// let speakers: Result<Speaker, _> = slice.iter().copied().map(Speaker::try_from).collect();
    /// let speakers = speakers.expect("invalid speaker value");
    /// ```
    ///
    /// # Safety
    ///
    /// `ASIOChannelList` must be valid for reads up to `ASIONumChannels`.
    /// Every pointer inside `ASIOChannelList` must be a null-terminated and must be valid for reads of bytes up to and including the nul terminator.
    ///
    ///
    /// See [`Utf8CStr::from_ptr_unchecked`] for more information.
    ///
    /// # Panics
    ///
    /// This function will panic if `resamplerMethod` is not a valid user resampler.
    pub unsafe fn from_ffi(value: FMOD_ADVANCEDSETTINGS) -> Self {
        let channels = if value.ASIONumChannels > 0 {
            let slice = unsafe {
                std::slice::from_raw_parts(value.ASIOChannelList, value.ASIONumChannels as _)
            };
            let vec = slice
                .iter()
                .map(|&ptr| unsafe { Utf8CStr::from_ptr_unchecked(ptr) }.to_cstring())
                .collect();
            Some(vec)
        } else {
            None
        };

        Self {
            max_mpeg_codecs: value.maxMPEGCodecs,
            max_adpcm_codecs: value.maxADPCMCodecs,
            max_xma_codecs: value.maxXMACodecs,
            max_vorbis_codecs: value.maxVorbisCodecs,
            max_at9_codecs: value.maxAT9Codecs,
            max_fadpcm_codecs: value.maxFADPCMCodecs,
            max_opus_codecs: value.maxOpusCodecs,
            #[cfg(fmod_lt_2_3)]
            max_pcm_codecs: value.maxPCMCodecs,

            asio_channel_list: channels,
            asio_speaker_list: None,

            vol0_virtual_vol: value.vol0virtualvol,
            default_decode_buffer_size: value.defaultDecodeBufferSize,
            profile_port: value.profilePort,
            geometry_max_fade_time: value.geometryMaxFadeTime,
            distance_filter_center_freq: value.distanceFilterCenterFreq,
            reverb_3d_instance: value.reverb3Dinstance,
            dsp_buffer_pool_size: value.DSPBufferPoolSize,
            resampler_method: value.resamplerMethod.try_into().unwrap(),
            random_seed: value.randomSeed,
            max_convolution_threads: value.maxConvolutionThreads,
            max_spatial_objects: value.maxSpatialObjects,
        }
    }
}
