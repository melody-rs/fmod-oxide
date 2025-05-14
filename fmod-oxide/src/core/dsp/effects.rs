use crate::{
    AttenuationRange as AttenuationRangeType, Attributes3DMulti, Dsp, DspType, DynamicResponse,
    Fft, OverallGain as OverallGainType, ReadableParameter, ReadableParameterIndex, Sidechain,
    SpeakerMode as SpeakerModeType, WritableParameter, WritableParameterIndex,
};

use crate::{Error, Result};
use fmod_sys::*;
use std::ffi::{c_float, c_int, c_short};
use std::mem::MaybeUninit;

// I really need to find better names for these.

macro_rules! dsp_param_impl {
    ($kind:ident =>  struct $name:ident($index:expr): $type:ty) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name;

        impl ReadableParameterIndex<$type> for $name {
            const TYPE: DspType = DspType::$kind;

            fn into_index(self) -> c_int {
                $index as c_int
            }
        }

        impl WritableParameterIndex<$type> for $name {
            const TYPE: DspType = DspType::$kind;

            fn into_index(self) -> c_int {
                $index as c_int
            }
        }
    };
}

macro_rules! read_dsp_param_impl {
    ($kind:ident =>  struct $name:ident($index:expr): $type:ty) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $name;

        impl ReadableParameterIndex<$type> for $name {
            const TYPE: DspType = DspType::$kind;

            fn into_index(self) -> c_int {
                $index as c_int
            }
        }
    };
}

macro_rules! enum_dsp_param_impl {
    ($name:ident: $repr:ty) => {
        impl ReadableParameter for $name {
            fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
                let value: c_int = dsp.get_parameter(index)?;
                Self::try_from(value as $repr).map_err(Into::into)
            }

            fn get_parameter_string(dsp: Dsp, index: c_int) -> Result<lanyard::Utf8CString> {
                dsp.get_parameter_string::<c_int, c_int>(index)
            }
        }

        impl WritableParameter for $name {
            fn set_parameter(self, dsp: Dsp, index: c_int) -> Result<()> {
                dsp.set_parameter(index, self as c_int)
            }
        }
    };
}

enum_dsp_param_impl!(SpeakerModeType: u32);

pub mod channel_mix {
    use super::*;

    dsp_param_impl!( ChannelMix => struct OutputGrouping(FMOD_DSP_CHANNELMIX_OUTPUTGROUPING): Output);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
    #[repr(u32)]
    pub enum Output {
        Default = FMOD_DSP_CHANNELMIX_OUTPUT_DEFAULT,
        AllMono = FMOD_DSP_CHANNELMIX_OUTPUT_ALLMONO,
        AllStereo = FMOD_DSP_CHANNELMIX_OUTPUT_ALLSTEREO,
        AllQuad = FMOD_DSP_CHANNELMIX_OUTPUT_ALLQUAD,
        All5Point7 = FMOD_DSP_CHANNELMIX_OUTPUT_ALL5POINT1,
        All7Point1 = FMOD_DSP_CHANNELMIX_OUTPUT_ALL7POINT1,
        AllLFE = FMOD_DSP_CHANNELMIX_OUTPUT_ALLLFE,
        All7Point4 = FMOD_DSP_CHANNELMIX_OUTPUT_ALL7POINT1POINT4,
    }

    enum_dsp_param_impl!(Output: u32);

    // Make this an enum? Constrain N somehow?
    #[derive(Debug, Clone, Copy)]
    pub struct GainChannel<const N: c_int>;

    impl<const N: c_int> ReadableParameterIndex<c_float> for GainChannel<N> {
        const TYPE: DspType = DspType::ChannelMix;

        fn into_index(self) -> c_int {
            FMOD_DSP_CHANNELMIX_GAIN_CH0 as c_int + N
        }
    }

    impl<const N: c_int> WritableParameterIndex<c_float> for GainChannel<N> {
        const TYPE: DspType = DspType::ChannelMix;

        fn into_index(self) -> c_int {
            FMOD_DSP_CHANNELMIX_GAIN_CH0 as c_int + N
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct OutputChannel<const N: c_int>;

    impl<const N: c_int> ReadableParameterIndex<c_int> for OutputChannel<N> {
        const TYPE: DspType = DspType::ChannelMix;

        fn into_index(self) -> c_int {
            FMOD_DSP_CHANNELMIX_OUTPUT_CH0 as c_int + N
        }
    }

    impl<const N: c_int> WritableParameterIndex<c_float> for OutputChannel<N> {
        const TYPE: DspType = DspType::ChannelMix;

        fn into_index(self) -> c_int {
            FMOD_DSP_CHANNELMIX_GAIN_CH0 as c_int + N
        }
    }
}

pub mod chorus {
    use super::*;

    dsp_param_impl!(Chorus => struct Mix(FMOD_DSP_CHORUS_MIX): c_float);
    dsp_param_impl!(Chorus => struct Rate(FMOD_DSP_CHORUS_RATE): c_float);
    dsp_param_impl!(Chorus => struct Depth(FMOD_DSP_CHORUS_DEPTH): c_float);
}

pub mod compressor {
    use super::*;

    dsp_param_impl!(Compressor => struct Threshold(FMOD_DSP_COMPRESSOR_THRESHOLD): c_float);
    dsp_param_impl!(Compressor => struct Ratio(FMOD_DSP_COMPRESSOR_RATIO): c_float);
    dsp_param_impl!(Compressor => struct Attack(FMOD_DSP_COMPRESSOR_ATTACK): c_float);
    dsp_param_impl!(Compressor => struct Release(FMOD_DSP_COMPRESSOR_RELEASE): c_float);
    dsp_param_impl!(Compressor => struct GainMakeup(FMOD_DSP_COMPRESSOR_GAINMAKEUP): c_float);
    dsp_param_impl!(Compressor => struct UseSideChain(FMOD_DSP_COMPRESSOR_USESIDECHAIN): Sidechain);
    dsp_param_impl!(Compressor => struct Linked(FMOD_DSP_COMPRESSOR_LINKED): bool);
}

pub mod convolution_reverb {
    use super::*;
    use crate::Sound;

    #[derive(Debug)]
    #[repr(transparent)]
    pub struct ImpulseResponse {
        data: [c_short],
    }

    impl ImpulseResponse {
        pub fn new(data: &[c_short]) -> &Self {
            unsafe { &*(std::ptr::from_ref::<[c_short]>(data) as *const Self) }
        }

        pub fn channel_count(&self) -> c_short {
            self.data[0]
        }

        pub fn data(&self) -> &[c_short] {
            &self.data[1..]
        }

        /// # Safety
        ///
        /// This function uses [`Sound::read_data`], which is *unsafe* if [`Sound::release`] is called from another thread while [`Sound::read_data`] is processing.
        pub unsafe fn from_sound(sound: Sound) -> Result<Box<Self>> {
            let (_, format, channels, _) = sound.get_format()?;
            if format != crate::SoundFormat::PCM16 {
                return Err(Error::InvalidParam);
            }

            let data_length = sound.get_length(crate::TimeUnit::PCM)?;
            let mut data = vec![0_i16; data_length as usize * channels as usize + 1];
            data[0] = channels as _;

            unsafe {
                sound.read_data(bytemuck::cast_slice_mut(&mut data[1..]))?;
            }

            let data = data.into_boxed_slice();
            Ok(unsafe { std::mem::transmute::<Box<[i16]>, Box<ImpulseResponse>>(data) })
        }
    }

    impl WritableParameter for &ImpulseResponse {
        fn set_parameter(self, dsp: Dsp, index: c_int) -> Result<()> {
            if dsp.get_type()? != DspType::ConvolutionReverb
                || index != FMOD_DSP_CONVOLUTION_REVERB_PARAM_IR as i32
            {
                return Err(Error::InvalidParam);
            }
            unsafe { dsp.set_raw_parameter_data::<[std::ffi::c_short]>(&self.data, index) }
        }
    }

    impl ReadableParameter for Box<ImpulseResponse> {
        fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
            if dsp.get_type()? != DspType::ConvolutionReverb
                || index != FMOD_DSP_CONVOLUTION_REVERB_PARAM_IR as i32
            {
                return Err(Error::InvalidParam);
            }

            let raw_data = unsafe { dsp.get_raw_parameter_data_slice(index) }?.to_vec();
            let data: Box<[c_short]> = bytemuck::cast_vec(raw_data).into_boxed_slice();
            Ok(unsafe { std::mem::transmute::<Box<[i16]>, Box<ImpulseResponse>>(data) })
        }

        fn get_parameter_string(dsp: Dsp, index: c_int) -> Result<lanyard::Utf8CString> {
            dsp.get_data_parameter_string(index)
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct IR;

    impl ReadableParameterIndex<Box<ImpulseResponse>> for IR {
        const TYPE: DspType = DspType::ConvolutionReverb;
        fn into_index(self) -> c_int {
            FMOD_DSP_CONVOLUTION_REVERB_PARAM_IR as c_int
        }
    }

    impl WritableParameterIndex<&ImpulseResponse> for IR {
        const TYPE: DspType = DspType::ConvolutionReverb;
        fn into_index(self) -> c_int {
            FMOD_DSP_CONVOLUTION_REVERB_PARAM_IR as c_int
        }
    }

    dsp_param_impl!(ConvolutionReverb => struct Wet(FMOD_DSP_CONVOLUTION_REVERB_PARAM_WET): c_float);
    dsp_param_impl!(ConvolutionReverb => struct Dry(FMOD_DSP_CONVOLUTION_REVERB_PARAM_DRY): c_float);
    dsp_param_impl!(ConvolutionReverb => struct ReleaLinkedse(FMOD_DSP_CONVOLUTION_REVERB_PARAM_LINKED): bool);
}

pub mod delay {
    use super::*;

    // Make this an enum? Constrain N somehow?
    #[derive(Debug, Clone, Copy)]
    pub struct Channel<const N: c_int>;

    impl<const N: c_int> ReadableParameterIndex<c_float> for Channel<N> {
        const TYPE: DspType = DspType::Delay;

        fn into_index(self) -> c_int {
            FMOD_DSP_DELAY_CH0 as c_int + N
        }
    }

    impl<const N: c_int> WritableParameterIndex<c_float> for Channel<N> {
        const TYPE: DspType = DspType::Delay;

        fn into_index(self) -> c_int {
            FMOD_DSP_DELAY_CH0 as c_int + N
        }
    }

    dsp_param_impl!(Delay => struct MaxDelay(FMOD_DSP_DELAY_MAXDELAY): c_float);
}

pub mod distortion {
    use super::*;

    dsp_param_impl!(Distortion => struct Level(FMOD_DSP_DISTORTION_LEVEL): c_float);
}

pub mod echo {
    use super::*;

    dsp_param_impl!(Echo => struct Delay(FMOD_DSP_ECHO_DELAY): c_float);
    dsp_param_impl!(Echo => struct Feedback(FMOD_DSP_ECHO_FEEDBACK): c_float);
    dsp_param_impl!(Echo => struct DryLevel(FMOD_DSP_ECHO_DRYLEVEL): c_float);
    dsp_param_impl!(Echo => struct WetLevel(FMOD_DSP_ECHO_WETLEVEL): c_float);
    dsp_param_impl!(Echo => struct DelayChangeMode(FMOD_DSP_ECHO_DELAYCHANGEMODE): DelayType);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
    #[repr(u32)]
    pub enum DelayType {
        Fade = FMOD_DSP_ECHO_DELAYCHANGEMODE_FADE,
        Lerp = FMOD_DSP_ECHO_DELAYCHANGEMODE_LERP,
        None = FMOD_DSP_ECHO_DELAYCHANGEMODE_NONE,
    }

    enum_dsp_param_impl!(DelayType: u32);
}

pub mod fader {
    use super::*;

    dsp_param_impl!(Fader => struct Gain(FMOD_DSP_FADER_GAIN): c_float);
    dsp_param_impl!(Fader => struct OverallGain(FMOD_DSP_FADER_OVERALL_GAIN): OverallGainType);
}

pub mod fft {
    use super::*;

    dsp_param_impl!(Fft => struct WindowSize(FMOD_DSP_FFT_WINDOWSIZE): c_int);
    dsp_param_impl!(Fft => struct Window(FMOD_DSP_FFT_WINDOW): WindowType);
    dsp_param_impl!(Fft => struct BandStartFreq(FMOD_DSP_FFT_BAND_START_FREQ): c_float);
    dsp_param_impl!(Fft => struct BandStopFreq(FMOD_DSP_FFT_BAND_STOP_FREQ): c_float);
    read_dsp_param_impl!(Fft => struct SpectrumData(FMOD_DSP_FFT_SPECTRUMDATA): Fft);
    read_dsp_param_impl!(Fft => struct Rms(FMOD_DSP_FFT_RMS): c_float);
    read_dsp_param_impl!(Fft => struct SpectralCentroid(FMOD_DSP_FFT_SPECTRAL_CENTROID): c_float);
    dsp_param_impl!(Fft => struct ImmediateMode(FMOD_DSP_FFT_IMMEDIATE_MODE): bool);
    dsp_param_impl!(Fft => struct Downmix(FMOD_DSP_FFT_DOWNMIX): DownmixType);
    dsp_param_impl!(Fft => struct Channel(FMOD_DSP_FFT_CHANNEL): c_int);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
    #[repr(u32)]
    pub enum WindowType {
        Rect = FMOD_DSP_FFT_WINDOW_RECT,
        Triangle = FMOD_DSP_FFT_WINDOW_TRIANGLE,
        Hamming = FMOD_DSP_FFT_WINDOW_HAMMING,
        Hanning = FMOD_DSP_FFT_WINDOW_HANNING,
        Blackman = FMOD_DSP_FFT_WINDOW_BLACKMAN,
        BlackmanHarris = FMOD_DSP_FFT_WINDOW_BLACKMANHARRIS,
    }

    enum_dsp_param_impl!(WindowType: u32);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
    #[repr(u32)]
    pub enum DownmixType {
        None = FMOD_DSP_FFT_DOWNMIX_NONE,
        Mono = FMOD_DSP_FFT_DOWNMIX_MONO,
    }

    enum_dsp_param_impl!(DownmixType: u32);
}

pub mod flange {
    use super::*;

    dsp_param_impl!(Flange => struct Mix(FMOD_DSP_FLANGE_MIX): c_float);
    dsp_param_impl!(Flange => struct Depth(FMOD_DSP_FLANGE_DEPTH): c_float);
    dsp_param_impl!(Flange => struct Rate(FMOD_DSP_FLANGE_RATE): c_float);
}

pub mod highpass {
    use super::*;

    dsp_param_impl!(Highpass => struct Cutoff(FMOD_DSP_HIGHPASS_CUTOFF): c_float);
    dsp_param_impl!(Highpass => struct Resonance(FMOD_DSP_HIGHPASS_RESONANCE): c_float);
}

pub mod highpass_simple {
    use super::*;

    dsp_param_impl!(Highpass => struct Cutoff(FMOD_DSP_HIGHPASS_SIMPLE_CUTOFF): c_float);
}

pub mod it_echo {
    use super::*;

    dsp_param_impl!(ItEcho => struct WetDryMix(FMOD_DSP_ITECHO_WETDRYMIX): c_float);
    dsp_param_impl!(ItEcho => struct Feedback(FMOD_DSP_ITECHO_FEEDBACK): c_float);
    dsp_param_impl!(ItEcho => struct LeftDelay(FMOD_DSP_ITECHO_LEFTDELAY): c_float);
    dsp_param_impl!(ItEcho => struct RightDelay(FMOD_DSP_ITECHO_RIGHTDELAY): c_float);
    dsp_param_impl!(ItEcho => struct PanDelay(FMOD_DSP_ITECHO_PANDELAY): c_float); // FIXME fmod says this is not supported?
}

pub mod it_lowpass {
    use super::*;

    dsp_param_impl!(ItLowpass => struct Cutoff(FMOD_DSP_ITLOWPASS_CUTOFF): c_float);
    dsp_param_impl!(ItLowpass => struct Resonance(FMOD_DSP_ITLOWPASS_RESONANCE): c_float);
}

pub mod limiter {
    use super::*;

    dsp_param_impl!(Limiter => struct ReleaseTime(FMOD_DSP_LIMITER_RELEASETIME): c_float);
    dsp_param_impl!(Limiter => struct Ceiling(FMOD_DSP_LIMITER_CEILING): c_float);
    dsp_param_impl!(Limiter => struct MaximizerGain(FMOD_DSP_LIMITER_MAXIMIZERGAIN): c_float);
    dsp_param_impl!(Limiter => struct Mode(FMOD_DSP_LIMITER_MODE): bool);
}

pub mod loudness_meter {
    use super::*;

    dsp_param_impl!(LoudnessMeter => struct State(FMOD_DSP_LOUDNESS_METER_STATE): CurrentState);
    dsp_param_impl!(LoudnessMeter => struct Weighting(FMOD_DSP_LOUDNESS_METER_WEIGHTING): c_float);
    read_dsp_param_impl!(LoudnessMeter => struct Info(FMOD_DSP_LOUDNESS_METER_INFO): InfoData);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
    #[repr(i32)]
    pub enum CurrentState {
        ResetIntegrated = FMOD_DSP_LOUDNESS_METER_STATE_RESET_INTEGRATED,
        MaxPeak = FMOD_DSP_LOUDNESS_METER_STATE_RESET_MAXPEAK,
        ResetAll = FMOD_DSP_LOUDNESS_METER_STATE_RESET_ALL,
        Paused = FMOD_DSP_LOUDNESS_METER_STATE_PAUSED,
        Analyzing = FMOD_DSP_LOUDNESS_METER_STATE_ANALYZING,
    }

    enum_dsp_param_impl!(CurrentState: i32);

    #[derive(Debug, Clone, Copy, PartialEq)]
    #[repr(C)]
    pub struct InfoData {
        pub momentary_loudness: c_float,
        pub shortterm_loudness: c_float,
        pub integrated_loudness: c_float,
        pub loudness_10th_percentile: c_float,
        pub loudness_95th_percentile: c_float,
        pub loudness_histogram: [c_float; 66],
        pub max_true_peak: c_float,
        pub max_momentary_loudness: c_float,
    }

    impl ReadableParameter for InfoData {
        fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
            if dsp.get_type()? != DspType::LoudnessMeter
                || index != FMOD_DSP_LOUDNESS_METER_INFO as i32
            {
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

    #[derive(Debug, Clone, Copy, PartialEq)]
    #[repr(C)]
    pub struct WeightingData {
        pub channel_weight: [c_float; 32],
    }

    impl ReadableParameter for WeightingData {
        fn get_parameter(dsp: Dsp, index: c_int) -> Result<Self> {
            if dsp.get_type()? != DspType::LoudnessMeter
                || index != FMOD_DSP_LOUDNESS_METER_WEIGHTING as i32
            {
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

    impl WritableParameter for WeightingData {
        fn set_parameter(self, dsp: Dsp, index: c_int) -> Result<()> {
            if dsp.get_type()? != DspType::LoudnessMeter
                || index != FMOD_DSP_LOUDNESS_METER_WEIGHTING as i32
            {
                return Err(Error::InvalidParam);
            }
            unsafe { dsp.set_raw_parameter_data(&self, index) }
        }
    }
}

pub mod lowpass {
    use super::*;

    dsp_param_impl!(Lowpass => struct Cutoff(FMOD_DSP_LOWPASS_CUTOFF): c_float);
    dsp_param_impl!(Lowpass => struct Resonance(FMOD_DSP_LOWPASS_RESONANCE): c_float);
}

pub mod lowpass_simple {
    use super::*;

    dsp_param_impl!(LowpassSimple => struct Cutoff(FMOD_DSP_LOWPASS_SIMPLE_CUTOFF): c_float);
}

#[cfg(fmod_2_3)]
pub mod multiband_dynamics {
    use super::*;

    dsp_param_impl!(MultibandDynamics => struct LowerFrequency(FMOD_DSP_MULTIBAND_DYNAMICS_LOWER_FREQUENCY): c_float);
    dsp_param_impl!(MultibandDynamics => struct UpperFrequency(FMOD_DSP_MULTIBAND_DYNAMICS_UPPER_FREQUENCY): c_float);
    dsp_param_impl!(MultibandDynamics => struct Linked(FMOD_DSP_MULTIBAND_DYNAMICS_LINKED): bool);
    dsp_param_impl!(MultibandDynamics => struct UseSidechain(FMOD_DSP_MULTIBAND_DYNAMICS_USE_SIDECHAIN): Sidechain);

    // Wow! I love writing the same thing 3 times!
    dsp_param_impl!(MultibandDynamics => struct ModeA(FMOD_DSP_MULTIBAND_DYNAMICS_A_MODE): ModeType);
    dsp_param_impl!(MultibandDynamics => struct GainA(FMOD_DSP_MULTIBAND_DYNAMICS_A_GAIN): c_float);
    dsp_param_impl!(MultibandDynamics => struct ThresholdA(FMOD_DSP_MULTIBAND_DYNAMICS_A_THRESHOLD): c_float);
    dsp_param_impl!(MultibandDynamics => struct RatioA(FMOD_DSP_MULTIBAND_DYNAMICS_A_RATIO): c_float);
    dsp_param_impl!(MultibandDynamics => struct AttackA(FMOD_DSP_MULTIBAND_DYNAMICS_A_ATTACK): c_float);
    dsp_param_impl!(MultibandDynamics => struct ReleaseA(FMOD_DSP_MULTIBAND_DYNAMICS_A_RELEASE): c_float);
    dsp_param_impl!(MultibandDynamics => struct GainMakeupA(FMOD_DSP_MULTIBAND_DYNAMICS_A_GAIN_MAKEUP): c_float);
    dsp_param_impl!(MultibandDynamics => struct ResponseDataA(FMOD_DSP_MULTIBAND_DYNAMICS_A_RESPONSE_DATA): DynamicResponse);

    dsp_param_impl!(MultibandDynamics => struct ModeB(FMOD_DSP_MULTIBAND_DYNAMICS_B_MODE): ModeType);
    dsp_param_impl!(MultibandDynamics => struct GainB(FMOD_DSP_MULTIBAND_DYNAMICS_B_GAIN): c_float);
    dsp_param_impl!(MultibandDynamics => struct ThresholdB(FMOD_DSP_MULTIBAND_DYNAMICS_B_THRESHOLD): c_float);
    dsp_param_impl!(MultibandDynamics => struct RatioB(FMOD_DSP_MULTIBAND_DYNAMICS_B_RATIO): c_float);
    dsp_param_impl!(MultibandDynamics => struct AttackB(FMOD_DSP_MULTIBAND_DYNAMICS_B_ATTACK): c_float);
    dsp_param_impl!(MultibandDynamics => struct ReleaseB(FMOD_DSP_MULTIBAND_DYNAMICS_B_RELEASE): c_float);
    dsp_param_impl!(MultibandDynamics => struct GainMakeupB(FMOD_DSP_MULTIBAND_DYNAMICS_B_GAIN_MAKEUP): c_float);
    dsp_param_impl!(MultibandDynamics => struct ResponseDataB(FMOD_DSP_MULTIBAND_DYNAMICS_B_RESPONSE_DATA): DynamicResponse);

    dsp_param_impl!(MultibandDynamics => struct ModeC(FMOD_DSP_MULTIBAND_DYNAMICS_C_MODE): ModeType);
    dsp_param_impl!(MultibandDynamics => struct GainC(FMOD_DSP_MULTIBAND_DYNAMICS_C_GAIN): c_float);
    dsp_param_impl!(MultibandDynamics => struct ThresholdC(FMOD_DSP_MULTIBAND_DYNAMICS_C_THRESHOLD): c_float);
    dsp_param_impl!(MultibandDynamics => struct RatioC(FMOD_DSP_MULTIBAND_DYNAMICS_C_RATIO): c_float);
    dsp_param_impl!(MultibandDynamics => struct AttackC(FMOD_DSP_MULTIBAND_DYNAMICS_C_ATTACK): c_float);
    dsp_param_impl!(MultibandDynamics => struct ReleaseC(FMOD_DSP_MULTIBAND_DYNAMICS_C_RELEASE): c_float);
    dsp_param_impl!(MultibandDynamics => struct GainMakeupC(FMOD_DSP_MULTIBAND_DYNAMICS_C_GAIN_MAKEUP): c_float);
    dsp_param_impl!(MultibandDynamics => struct ResponseDataC(FMOD_DSP_MULTIBAND_DYNAMICS_C_RESPONSE_DATA): DynamicResponse);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
    #[repr(u32)]
    pub enum ModeType {
        Disabled = FMOD_DSP_MULTIBAND_DYNAMICS_MODE_DISABLED,
        CompressUp = FMOD_DSP_MULTIBAND_DYNAMICS_MODE_COMPRESS_UP,
        CompressDown = FMOD_DSP_MULTIBAND_DYNAMICS_MODE_COMPRESS_DOWN,
        ExpandUp = FMOD_DSP_MULTIBAND_DYNAMICS_MODE_EXPAND_UP,
        ExpandDown = FMOD_DSP_MULTIBAND_DYNAMICS_MODE_EXPAND_DOWN,
    }
    enum_dsp_param_impl!(ModeType: u32);
}

pub mod multiband_eq {
    use super::*;

    dsp_param_impl!(MultibandEq => struct FilterA(FMOD_DSP_MULTIBAND_EQ_A_FILTER): FilterType);
    dsp_param_impl!(MultibandEq => struct FrequencyA(FMOD_DSP_MULTIBAND_EQ_A_FREQUENCY): c_float);
    dsp_param_impl!(MultibandEq => struct QualityA(FMOD_DSP_MULTIBAND_EQ_A_Q): c_float);
    dsp_param_impl!(MultibandEq => struct GainA(FMOD_DSP_MULTIBAND_EQ_A_GAIN): c_float);

    dsp_param_impl!(MultibandEq => struct FilterB(FMOD_DSP_MULTIBAND_EQ_B_FILTER): FilterType);
    dsp_param_impl!(MultibandEq => struct FrequencyB(FMOD_DSP_MULTIBAND_EQ_B_FREQUENCY): c_float);
    dsp_param_impl!(MultibandEq => struct QualityB(FMOD_DSP_MULTIBAND_EQ_B_Q): c_float);
    dsp_param_impl!(MultibandEq => struct GainB(FMOD_DSP_MULTIBAND_EQ_B_GAIN): c_float);

    dsp_param_impl!(MultibandEq => struct FilterC(FMOD_DSP_MULTIBAND_EQ_C_FILTER): FilterType);
    dsp_param_impl!(MultibandEq => struct FrequencyC(FMOD_DSP_MULTIBAND_EQ_C_FREQUENCY): c_float);
    dsp_param_impl!(MultibandEq => struct QualityC(FMOD_DSP_MULTIBAND_EQ_C_Q): c_float);
    dsp_param_impl!(MultibandEq => struct GainC(FMOD_DSP_MULTIBAND_EQ_C_GAIN): c_float);

    dsp_param_impl!(MultibandEq => struct FilterD(FMOD_DSP_MULTIBAND_EQ_D_FILTER): FilterType);
    dsp_param_impl!(MultibandEq => struct FrequencyD(FMOD_DSP_MULTIBAND_EQ_D_FREQUENCY): c_float);
    dsp_param_impl!(MultibandEq => struct QualityD(FMOD_DSP_MULTIBAND_EQ_D_Q): c_float);
    dsp_param_impl!(MultibandEq => struct GainD(FMOD_DSP_MULTIBAND_EQ_D_GAIN): c_float);

    dsp_param_impl!(MultibandEq => struct FilterE(FMOD_DSP_MULTIBAND_EQ_E_FILTER): FilterType);
    dsp_param_impl!(MultibandEq => struct FrequencyE(FMOD_DSP_MULTIBAND_EQ_E_FREQUENCY): c_float);
    dsp_param_impl!(MultibandEq => struct QualityE(FMOD_DSP_MULTIBAND_EQ_E_Q): c_float);
    dsp_param_impl!(MultibandEq => struct GainE(FMOD_DSP_MULTIBAND_EQ_E_GAIN): c_float);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
    #[repr(u32)]
    pub enum FilterType {
        Disabled = FMOD_DSP_MULTIBAND_EQ_FILTER_DISABLED,
        Lowpass12DB = FMOD_DSP_MULTIBAND_EQ_FILTER_LOWPASS_12DB,
        Lowpass24DB = FMOD_DSP_MULTIBAND_EQ_FILTER_LOWPASS_24DB,
        Lowpass48DB = FMOD_DSP_MULTIBAND_EQ_FILTER_LOWPASS_48DB,
        Highpass12DB = FMOD_DSP_MULTIBAND_EQ_FILTER_HIGHPASS_12DB,
        Highpass24DB = FMOD_DSP_MULTIBAND_EQ_FILTER_HIGHPASS_24DB,
        Highpass48DB = FMOD_DSP_MULTIBAND_EQ_FILTER_HIGHPASS_48DB,
        LowShelf = FMOD_DSP_MULTIBAND_EQ_FILTER_LOWSHELF,
        HighShelf = FMOD_DSP_MULTIBAND_EQ_FILTER_HIGHSHELF,
        Peaking = FMOD_DSP_MULTIBAND_EQ_FILTER_PEAKING,
        BandPass = FMOD_DSP_MULTIBAND_EQ_FILTER_BANDPASS,
        Notch = FMOD_DSP_MULTIBAND_EQ_FILTER_NOTCH,
        AllPass = FMOD_DSP_MULTIBAND_EQ_FILTER_ALLPASS,
        Lowpass6DB = FMOD_DSP_MULTIBAND_EQ_FILTER_LOWPASS_6DB,
        Highpass6DB = FMOD_DSP_MULTIBAND_EQ_FILTER_HIGHPASS_6DB,
    }
    enum_dsp_param_impl!(FilterType: u32);
}

pub mod normalize {
    use super::*;

    dsp_param_impl!(Normalize => struct FadeTime(FMOD_DSP_NORMALIZE_FADETIME): c_float);
    dsp_param_impl!(Normalize => struct Threshold(FMOD_DSP_NORMALIZE_THRESHOLD): c_float);
    dsp_param_impl!(Normalize => struct MaxAmp(FMOD_DSP_NORMALIZE_MAXAMP): c_float);
}

pub mod object_pan {
    use super::pan::d3::{ExtentModeType, RolloffType};
    use super::*;

    dsp_param_impl!(ObjectPan => struct Position(FMOD_DSP_OBJECTPAN_3D_POSITION): Attributes3DMulti);
    dsp_param_impl!(ObjectPan => struct Rolloff(FMOD_DSP_OBJECTPAN_3D_ROLLOFF): RolloffType);
    dsp_param_impl!(ObjectPan => struct MinDistance(FMOD_DSP_OBJECTPAN_3D_MIN_DISTANCE): c_float);
    dsp_param_impl!(ObjectPan => struct MaxDistance(FMOD_DSP_OBJECTPAN_3D_MAX_DISTANCE): c_float);
    dsp_param_impl!(ObjectPan => struct ExtentMode(FMOD_DSP_OBJECTPAN_3D_EXTENT_MODE): ExtentModeType);
    dsp_param_impl!(ObjectPan => struct SoundSize(FMOD_DSP_OBJECTPAN_3D_SOUND_SIZE): c_float);
    dsp_param_impl!(ObjectPan => struct MinExtent(FMOD_DSP_OBJECTPAN_3D_MIN_EXTENT): c_float);
    read_dsp_param_impl!(ObjectPan => struct OverallGain(FMOD_DSP_OBJECTPAN_OVERALL_GAIN): OverallGainType);
    dsp_param_impl!(ObjectPan => struct OutputGain(FMOD_DSP_OBJECTPAN_OUTPUTGAIN): c_float);
    dsp_param_impl!(ObjectPan => struct AttenuationRange(FMOD_DSP_OBJECTPAN_ATTENUATION_RANGE): AttenuationRangeType);
    dsp_param_impl!(ObjectPan => struct OverrideRange(FMOD_DSP_OBJECTPAN_OVERRIDE_RANGE): bool);
}

pub mod oscillator {
    use super::*;

    dsp_param_impl!(Oscillator => struct Type(FMOD_DSP_OSCILLATOR_TYPE): OscillatorType);
    dsp_param_impl!(Oscillator => struct Rate(FMOD_DSP_OSCILLATOR_RATE): c_float);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
    #[repr(u32)]
    pub enum OscillatorType {
        Sine = 0,
        Square = 1,
        SawUp = 2,
        SawDown = 3,
        Triangle = 4,
        Noise = 5,
    }
    enum_dsp_param_impl!(OscillatorType: u32);
}

pub mod pan {
    use super::*;

    dsp_param_impl!(Pan => struct Mode(FMOD_DSP_PAN_MODE): ModeType);
    dsp_param_impl!(Pan => struct EnabledSpeakers(FMOD_DSP_PAN_ENABLED_SPEAKERS): c_int);
    dsp_param_impl!(Pan => struct LFEUpmixEnabled(FMOD_DSP_PAN_LFE_UPMIX_ENABLED): c_int); // FIXME this is just bool. but needs to be an int
    dsp_param_impl!(Pan => struct OverallGain(FMOD_DSP_PAN_OVERALL_GAIN): OverallGainType);
    dsp_param_impl!(Pan => struct SpeakerMode(FMOD_DSP_PAN_SURROUND_SPEAKER_MODE): SpeakerModeType);
    dsp_param_impl!(Pan => struct AttenuationRange(FMOD_DSP_PAN_ATTENUATION_RANGE): AttenuationRangeType);
    dsp_param_impl!(Pan => struct OverrideRange(FMOD_DSP_PAN_OVERRIDE_RANGE): bool);

    pub mod d3 {
        use super::super::*;

        dsp_param_impl!(Pan => struct Position(FMOD_DSP_PAN_3D_POSITION): Attributes3DMulti);
        dsp_param_impl!(Pan => struct Rolloff(FMOD_DSP_PAN_3D_ROLLOFF): RolloffType);
        dsp_param_impl!(Pan => struct MinDistance(FMOD_DSP_PAN_3D_MIN_DISTANCE): c_float);
        dsp_param_impl!(Pan => struct MaxDistance(FMOD_DSP_PAN_3D_MAX_DISTANCE): c_float);
        dsp_param_impl!(Pan => struct ExtentMode(FMOD_DSP_PAN_3D_MIN_DISTANCE): ExtentModeType);
        dsp_param_impl!(Pan => struct SoundSize(FMOD_DSP_PAN_3D_SOUND_SIZE): c_float);
        dsp_param_impl!(Pan => struct MinExtent(FMOD_DSP_PAN_3D_MIN_EXTENT): c_float);
        dsp_param_impl!(Pan => struct PanBlend(FMOD_DSP_PAN_3D_PAN_BLEND): c_float);

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
        #[repr(u32)]
        pub enum RolloffType {
            LinearSquared = FMOD_DSP_PAN_3D_ROLLOFF_LINEARSQUARED,
            Linear = FMOD_DSP_PAN_3D_ROLLOFF_LINEAR,
            Inverse = FMOD_DSP_PAN_3D_ROLLOFF_INVERSE,
            InverseTapered = FMOD_DSP_PAN_3D_ROLLOFF_INVERSETAPERED,
            Custom = FMOD_DSP_PAN_3D_ROLLOFF_CUSTOM,
        }
        enum_dsp_param_impl!(RolloffType: u32);

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
        #[repr(u32)]
        pub enum ExtentModeType {
            Auto = FMOD_DSP_PAN_3D_EXTENT_MODE_AUTO,
            User = FMOD_DSP_PAN_3D_EXTENT_MODE_USER,
            Off = FMOD_DSP_PAN_3D_EXTENT_MODE_OFF,
        }
        enum_dsp_param_impl!(ExtentModeType: u32);
    }
    pub mod d2 {
        use super::super::*;

        dsp_param_impl!(Pan => struct StereoPosition(FMOD_DSP_PAN_2D_STEREO_POSITION): c_float);
        dsp_param_impl!(Pan => struct Direction(FMOD_DSP_PAN_2D_DIRECTION): c_float);
        dsp_param_impl!(Pan => struct Extent(FMOD_DSP_PAN_2D_EXTENT): c_float);
        dsp_param_impl!(Pan => struct Rotation(FMOD_DSP_PAN_2D_ROTATION): c_float);
        dsp_param_impl!(Pan => struct LFELevel(FMOD_DSP_PAN_2D_LFE_LEVEL): c_float);
        dsp_param_impl!(Pan => struct StereoMode(FMOD_DSP_PAN_2D_STEREO_MODE): StereoModeType);
        dsp_param_impl!(Pan => struct StereoSeparation(FMOD_DSP_PAN_2D_STEREO_SEPARATION): c_float);
        dsp_param_impl!(Pan => struct StereoAxis(FMOD_DSP_PAN_2D_STEREO_AXIS): c_float);
        dsp_param_impl!(Pan => struct HeightBlend(FMOD_DSP_PAN_2D_HEIGHT_BLEND): c_float);

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
        #[repr(u32)]
        pub enum StereoModeType {
            Distributed = FMOD_DSP_PAN_2D_STEREO_MODE_DISTRIBUTED,
            Discrete = FMOD_DSP_PAN_2D_STEREO_MODE_DISCRETE,
        }
        enum_dsp_param_impl!(StereoModeType: u32);
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
    #[repr(u32)]
    pub enum ModeType {
        Mono = FMOD_DSP_PAN_MODE_MONO,
        Stereo = FMOD_DSP_PAN_MODE_STEREO,
        Surround = FMOD_DSP_PAN_MODE_SURROUND,
    }
    enum_dsp_param_impl!(ModeType: u32);
}

pub mod param_eq {
    use super::*;

    dsp_param_impl!(ParamEq => struct Center(FMOD_DSP_PARAMEQ_CENTER): c_float);
    dsp_param_impl!(ParamEq => struct Bandwith(FMOD_DSP_PARAMEQ_BANDWIDTH): c_float);
    dsp_param_impl!(ParamEq => struct Gain(FMOD_DSP_PARAMEQ_GAIN): c_float);
}

pub mod return_dsp {
    use super::*;

    read_dsp_param_impl!(Return => struct Id(FMOD_DSP_RETURN_ID): c_int);
    dsp_param_impl!(Return => struct SpeakerMode(FMOD_DSP_RETURN_INPUT_SPEAKER_MODE): SpeakerModeType);
}

pub mod send {
    use super::*;

    read_dsp_param_impl!(Send => struct Id(FMOD_DSP_SEND_RETURNID): c_int);
    dsp_param_impl!(Send => struct Level(FMOD_DSP_SEND_LEVEL): c_float);
}

pub mod sfx_reverb {
    use super::*;

    dsp_param_impl!(SfxReverb => struct DecayTime(FMOD_DSP_SFXREVERB_DECAYTIME): c_float);
    dsp_param_impl!(SfxReverb => struct EarlyDelay(FMOD_DSP_SFXREVERB_EARLYDELAY): c_float);
    dsp_param_impl!(SfxReverb => struct LateDelay(FMOD_DSP_SFXREVERB_LATEDELAY): c_float);
    dsp_param_impl!(SfxReverb => struct HFReference(FMOD_DSP_SFXREVERB_HFREFERENCE): c_float);
    dsp_param_impl!(SfxReverb => struct HFDecayRatio(FMOD_DSP_SFXREVERB_HFDECAYRATIO): c_float);
    dsp_param_impl!(SfxReverb => struct Diffusion(FMOD_DSP_SFXREVERB_DIFFUSION): c_float);
    dsp_param_impl!(SfxReverb => struct Density(FMOD_DSP_SFXREVERB_DENSITY): c_float);
    dsp_param_impl!(SfxReverb => struct LowShelfFrequency(FMOD_DSP_SFXREVERB_LOWSHELFFREQUENCY): c_float);
    dsp_param_impl!(SfxReverb => struct LowShelfGain(FMOD_DSP_SFXREVERB_LOWSHELFGAIN): c_float);
    dsp_param_impl!(SfxReverb => struct HighCut(FMOD_DSP_SFXREVERB_HIGHCUT): c_float);
    dsp_param_impl!(SfxReverb => struct EarlyLateMix(FMOD_DSP_SFXREVERB_EARLYLATEMIX): c_float);
    dsp_param_impl!(SfxReverb => struct WetLevel(FMOD_DSP_SFXREVERB_WETLEVEL): c_float);
    dsp_param_impl!(SfxReverb => struct DryLevel(FMOD_DSP_SFXREVERB_DRYLEVEL): c_float);
}

pub mod three_eq {
    use super::*;

    dsp_param_impl!(ThreeEq => struct LowGain(FMOD_DSP_THREE_EQ_LOWGAIN): c_float);
    dsp_param_impl!(ThreeEq => struct MidGain(FMOD_DSP_THREE_EQ_MIDGAIN): c_float);
    dsp_param_impl!(ThreeEq => struct HighGain(FMOD_DSP_THREE_EQ_HIGHGAIN): c_float);
    dsp_param_impl!(ThreeEq => struct LowCrossover(FMOD_DSP_THREE_EQ_LOWCROSSOVER): c_float);
    dsp_param_impl!(ThreeEq => struct HighCrossover(FMOD_DSP_THREE_EQ_HIGHCROSSOVER): c_float);
    dsp_param_impl!(ThreeEq => struct CrossoverSlope(FMOD_DSP_THREE_EQ_CROSSOVERSLOPE): CrossoverSlopeType);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
    #[repr(u32)]
    pub enum CrossoverSlopeType {
        _12DB = FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_12DB,
        _24DB = FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_24DB,
        _48DB = FMOD_DSP_THREE_EQ_CROSSOVERSLOPE_48DB,
    }
    enum_dsp_param_impl!(CrossoverSlopeType: u32);
}

pub mod transceiver {
    use super::*;

    dsp_param_impl!(Transceiver => struct Transmit(FMOD_DSP_TRANSCEIVER_TRANSMIT): bool);
    dsp_param_impl!(Transceiver => struct Gain(FMOD_DSP_TRANSCEIVER_GAIN): c_float);
    dsp_param_impl!(Transceiver => struct Channel(FMOD_DSP_TRANSCEIVER_CHANNEL): c_int);
    dsp_param_impl!(Transceiver => struct TransmitSpeakerMode(FMOD_DSP_TRANSCEIVER_TRANSMITSPEAKERMODE): SpeakerMode);

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
    #[repr(i32)]
    pub enum SpeakerMode {
        Auto = FMOD_DSP_TRANSCEIVER_SPEAKERMODE_AUTO,
        Mono = FMOD_DSP_TRANSCEIVER_SPEAKERMODE_MONO,
        Stereo = FMOD_DSP_TRANSCEIVER_SPEAKERMODE_STEREO,
        Surround = FMOD_DSP_TRANSCEIVER_SPEAKERMODE_SURROUND,
    }
    enum_dsp_param_impl!(SpeakerMode: i32);
}

pub mod tremolo {
    use super::*;

    dsp_param_impl!(Tremolo => struct Frequency(FMOD_DSP_TREMOLO_FREQUENCY): c_float);
    dsp_param_impl!(Tremolo => struct Depth(FMOD_DSP_TREMOLO_DEPTH): c_float);
    dsp_param_impl!(Tremolo => struct Shape(FMOD_DSP_TREMOLO_SHAPE): c_float);
    dsp_param_impl!(Tremolo => struct Skew(FMOD_DSP_TREMOLO_SKEW): c_float);
    dsp_param_impl!(Tremolo => struct Duty(FMOD_DSP_TREMOLO_DUTY): c_float);
    dsp_param_impl!(Tremolo => struct Square(FMOD_DSP_TREMOLO_SQUARE): c_float);
    dsp_param_impl!(Tremolo => struct Phase(FMOD_DSP_TREMOLO_PHASE): c_float);
    dsp_param_impl!(Tremolo => struct Spread(FMOD_DSP_TREMOLO_SPREAD): c_float);
}

#[cfg(fmod_2_2)]
pub mod envelope_follower {
    use super::*;

    dsp_param_impl!(EnvelopeFollower => struct Attack(FMOD_DSP_ENVELOPEFOLLOWER_ATTACK): c_float);
    dsp_param_impl!(EnvelopeFollower => struct Release(FMOD_DSP_ENVELOPEFOLLOWER_RELEASE): c_float);
    dsp_param_impl!(EnvelopeFollower => struct Envelope(FMOD_DSP_ENVELOPEFOLLOWER_ENVELOPE): c_float);
    dsp_param_impl!(EnvelopeFollower => struct UseSidechain(FMOD_DSP_ENVELOPEFOLLOWER_USESIDECHAIN): Sidechain);
}
