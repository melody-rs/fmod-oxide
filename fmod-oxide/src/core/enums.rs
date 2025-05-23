// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{Error, Result};
use fmod_sys::*;

#[cfg(doc)]
use crate::{Channel, ChannelControl, Dsp, Geometry, Sound, System, SystemBuilder, studio};

/// Speaker mode types.
///
/// Note below the phrase 'sound channels' is used. These are the subchannels inside a sound, they are not related and have nothing to do with the FMOD class "Channel".
///
/// For example a mono sound has 1 sound channel, a stereo sound has 2 sound channels, and an AC3 or 6 channel wav file have 6 "sound channels".
///
/// [`FMOD_SPEAKERMODE_RAW`]
/// This mode is for output devices that are not specifically mono/stereo/quad/surround/5.1 or 7.1, but are multi-channel.
/// - Use [`SystemBuilder::software_format`] to specify the number of speakers you want to address, otherwise it will default to 2 (stereo).
/// - Sound channels map to speakers sequentially, so a mono sound maps to output speaker 0, stereo sound maps to output speaker 0 & 1.
/// - The user assumes knowledge of the speaker order. [`Speaker`] enumerations may not apply, so raw channel indices should be used.
/// - Multi-channel sounds map input channels to output channels 1:1.
/// - Speaker levels must be manually set with [`ChannelControl::set_mix_matrix`].
/// - [`ChannelControl::set_pan`] and [`ChannelControl::set_mix_levels_output`] do not work.
///
/// [`FMOD_SPEAKERMODE_MONO`]
/// This mode is for a 1 speaker arrangement.
///
/// - Panning does not work in this speaker mode.
/// - Mono, stereo and multi-channel sounds have each sound channel played on the one speaker at unity.
/// - Mix behavior for multi-channel sounds can be set with [`ChannelControl::set_mix_matrix`].
///
/// [`FMOD_SPEAKERMODE_STEREO`]
/// This mode is for 2 speaker arrangements that have a left and right speaker.
///
/// - Mono sounds default to an even distribution between left and right. They can be panned with [`ChannelControl::set_pan`].
/// - Stereo sounds default to the middle, or full left in the left speaker and full right in the right speaker.
///   They can be cross faded with [`ChannelControl::set_pan`].
/// - Multi-channel sounds have each sound channel played on each speaker at unity.
/// - Mix behavior for multi-channel sounds can be set with [`ChannelControl::set_mix_matrix`].
///
/// [`FMOD_SPEAKERMODE_QUAD`]
/// This mode is for 4 speaker arrangements that have a front left, front right, surround left and a surround right speaker.
///
/// - Mono sounds default to an even distribution between front left and front right. They can be panned with [`ChannelControl::set_pan`].
/// - Stereo sounds default to the left sound channel played on the front left, and the right sound channel played on the front right.
///   They can be cross faded with [`ChannelControl::set_pan`].
/// - Multi-channel sounds default to all of their sound channels being played on each speaker in order of input.
/// - Mix behavior for multi-channel sounds can be set with [`ChannelControl::set_mix_matrix`].
///
/// [`FMOD_SPEAKERMODE_SURROUND`]
/// This mode is for 5 speaker arrangements that have a left/right/center/surround left/surround right.
///
/// - Mono sounds default to the center speaker. They can be panned with [`ChannelControl::set_pan`].
/// - Stereo sounds default to the left sound channel played on the front left, and the right sound channel played on the front right.
///   They can be cross faded with [`ChannelControl::set_pan`].
/// - Multi-channel sounds default to all of their sound channels being played on each speaker in order of input.
/// - Mix behavior for multi-channel sounds can be set with [`ChannelControl::set_mix_matrix`].
///
/// [`FMOD_SPEAKERMODE_5POINT1`]
/// This mode is for 5.1 speaker arrangements that have a left/right/center/surround left/surround right and a subwoofer speaker.
///
/// - Mono sounds default to the center speaker. They can be panned with [`ChannelControl::set_pan`].
/// - Stereo sounds default to the left sound channel played on the front left, and the right sound channel played on the front right.
///   They can be cross faded with [`ChannelControl::set_pan`].
/// - Multi-channel sounds default to all of their sound channels being played on each speaker in order of input.
/// - Mix behavior for multi-channel sounds can be set with [`ChannelControl::set_mix_matrix`].
///
/// [`FMOD_SPEAKERMODE_7POINT1`]
/// This mode is for 7.1 speaker arrangements that have a left/right/center/surround left/surround right/rear left/rear right and a subwoofer speaker.
///
/// - Mono sounds default to the center speaker. They can be panned with [`ChannelControl::set_pan`].
/// - Stereo sounds default to the left sound channel played on the front left, and the right sound channel played on the front right.
///   They can be cross faded with [`ChannelControl::set_pan`].
/// - Multi-channel sounds default to all of their sound channels being played on each speaker in order of input.
/// - Mix behavior for multi-channel sounds can be set with [`ChannelControl::set_mix_matrix`].
///
/// See the FMOD Studio Mixing Guide for graphical depictions of each speaker mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum SpeakerMode {
    /// Default speaker mode for the chosen output mode which will resolve after [`SystemBuilder::build`].
    Default = FMOD_SPEAKERMODE_DEFAULT,
    /// Assume there is no special mapping from a given channel to a speaker, channels map 1:1 in order.
    ///
    /// Use [`SystemBuilder::software_format`] to specify the speaker count.
    Raw = FMOD_SPEAKERMODE_RAW,
    /// 1 speaker setup (monaural).
    Mono = FMOD_SPEAKERMODE_MONO,
    /// 2 speaker setup (stereo) front left, front right.
    Stereo = FMOD_SPEAKERMODE_STEREO,
    /// 4 speaker setup (4.0) front left, front right, surround left, surround right.
    Quad = FMOD_SPEAKERMODE_QUAD,
    /// 5 speaker setup (5.0) front left, front right, center, surround left, surround right.
    Surround = FMOD_SPEAKERMODE_SURROUND,
    /// 6 speaker setup (5.1) front left, front right, center, low frequency, surround left, surround right.
    FivePointOne = FMOD_SPEAKERMODE_5POINT1,
    /// 8 speaker setup (7.1) front left, front right, center, low frequency, surround left, surround right, back left, back right.
    SevenPointOne = FMOD_SPEAKERMODE_7POINT1,
    /// 12 speaker setup (7.1.4) front left, front right, center, low frequency, surround left, surround right, back left, back right, top front left, top front right, top back left, top back right.
    SevenPointOneFour = FMOD_SPEAKERMODE_7POINT1POINT4,
}

/// Built-in output types that can be used to run the mixer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum OutputType {
    /// Picks the best output mode for the platform. This is the default.
    AutoDetect = FMOD_OUTPUTTYPE_AUTODETECT,
    /// All - 3rd party plug-in, unknown.
    /// This is for use with [`System::get_output_type`] only.
    Unknown = FMOD_OUTPUTTYPE_UNKNOWN,
    /// All - Perform all mixing but discard the final output.
    NoSound = FMOD_OUTPUTTYPE_NOSOUND,
    /// All - Writes output to a .wav file.
    WavWriter = FMOD_OUTPUTTYPE_WAVWRITER,
    /// All - Non-realtime version of [`FMOD_OUTPUTTYPE_NOSOUND`], one mix per [`System::update`].
    NoSoundNRT = FMOD_OUTPUTTYPE_NOSOUND_NRT,
    /// All - Non-realtime version of [`FMOD_OUTPUTTYPE_WAVWRITER`], one mix per [`System::update`].
    WavWriterNRT = FMOD_OUTPUTTYPE_WAVWRITER_NRT,
    /// Win / UWP / Xbox One / Game Core - Windows Audio Session API.
    ///
    /// (Default on Windows, Xbox One, Game Core and UWP)
    WASAPI = FMOD_OUTPUTTYPE_WASAPI,
    /// Win - Low latency ASIO 2.0.
    ASIO = FMOD_OUTPUTTYPE_ASIO,
    /// Linux - Pulse Audio.
    ///
    /// (Default on Linux if available)
    PulseAudio = FMOD_OUTPUTTYPE_PULSEAUDIO,
    /// Linux - Advanced Linux Sound Architecture.
    ///
    /// (Default on Linux if `PulseAudio` isn't available)
    Alsa = FMOD_OUTPUTTYPE_ALSA,
    /// Mac / iOS - Core Audio. (Default on Mac and iOS)
    CoreAudio = FMOD_OUTPUTTYPE_COREAUDIO,
    /// Android - Java Audio Track.
    ///
    /// (Default on Android 2.2 and below)
    AudioTrack = FMOD_OUTPUTTYPE_AUDIOTRACK,
    /// Android - `OpenSL` ES.
    ///
    /// (Default on Android 2.3 up to 7.1)
    OpenSL = FMOD_OUTPUTTYPE_OPENSL,
    /// PS4 / PS5 - Audio Out.
    ///
    /// (Default on PS4, PS5)
    AudioOut = FMOD_OUTPUTTYPE_AUDIOOUT,
    /// PS4 - `Audio3D`.
    Audio3D = FMOD_OUTPUTTYPE_AUDIO3D,
    /// HTML5 - Web Audio `ScriptProcessorNode` output.
    ///
    /// (Default on HTML5 if `AudioWorkletNode` isn't available)
    WebAudio = FMOD_OUTPUTTYPE_WEBAUDIO,
    /// Switch - `nn::audio`.
    ///
    /// (Default on Switch)
    NNAudio = FMOD_OUTPUTTYPE_NNAUDIO,
    /// Win10 / Xbox One / Game Core - Windows Sonic.
    WinSonic = FMOD_OUTPUTTYPE_WINSONIC,
    /// Android - `AAudio`.
    ///
    /// (Default on Android 8.1 and above)
    AAudio = FMOD_OUTPUTTYPE_AAUDIO,
    /// HTML5 - Web Audio `AudioWorkletNode` output.
    ///
    /// (Default on HTML5 if available)
    AudioWorklet = FMOD_OUTPUTTYPE_AUDIOWORKLET,
    /// iOS - PHASE framework.
    ///
    /// (Disabled)
    Phase = FMOD_OUTPUTTYPE_PHASE,
    /// `OpenHarmony` - `OHAudio`.
    OHAudio = FMOD_OUTPUTTYPE_OHAUDIO,
}

/// Named constants for threads created at runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum ThreadType {
    /// Thread responsible for mixing and processing blocks of audio.
    Mixer = FMOD_THREAD_TYPE_MIXER,
    /// Thread used by some output plug-ins for transferring buffered audio from [`FMOD_THREAD_TYPE_MIXER`] to the sound output device.
    Feeder = FMOD_THREAD_TYPE_FEEDER,
    /// Thread that decodes compressed audio to PCM for Sounds created as [`FMOD_CREATESTREAM`].
    Stream = FMOD_THREAD_TYPE_STREAM,
    /// Thread that reads compressed audio from disk to be consumed by [`FMOD_THREAD_TYPE_STREAM`].
    File = FMOD_THREAD_TYPE_FILE,
    /// Thread that processes the creation of Sounds asynchronously when opened with [`FMOD_NONBLOCKING`].
    NonBlocking = FMOD_THREAD_TYPE_NONBLOCKING,
    /// Thread used by some output plug-ins for transferring audio from a microphone to [`FMOD_THREAD_TYPE_MIXER`].
    Record = FMOD_THREAD_TYPE_RECORD,
    /// Thread used by the [`Geometry`] system for performing background calculations.
    Geometry = FMOD_THREAD_TYPE_GEOMETRY,
    /// Thread for network communication when using [`FMOD_INIT_PROFILE_ENABLE`].
    Profiler = FMOD_THREAD_TYPE_PROFILER,
    /// Thread for processing Studio API commands and scheduling sound playback.
    StudioUpdate = FMOD_THREAD_TYPE_STUDIO_UPDATE,
    /// Thread for asynchronously loading [`studio::Bank`] metadata.
    StudioLoadBank = FMOD_THREAD_TYPE_STUDIO_LOAD_BANK,
    /// Thread for asynchronously loading [`studio::Bank`] sample data.
    StudioLoadSample = FMOD_THREAD_TYPE_STUDIO_LOAD_SAMPLE,
    /// Thread for processing medium size delay lines for [`FMOD_DSP_TYPE_CONVOLUTIONREVERB`].
    Convolution1 = FMOD_THREAD_TYPE_CONVOLUTION1,
    /// Thread for processing larger size delay lines for [`FMOD_DSP_TYPE_CONVOLUTIONREVERB`].
    Convolution2 = FMOD_THREAD_TYPE_CONVOLUTION2,
}

/// Time types used for position or length.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
#[repr(u32)]
pub enum TimeUnit {
    /// Milliseconds.
    MS = FMOD_TIMEUNIT_MS,
    /// PCM samples, related to milliseconds * samplerate / 1000.
    PCM = FMOD_TIMEUNIT_PCM,
    /// Bytes, related to PCM samples * channels * datawidth (ie 16bit = 2 bytes).
    PCMBytes = FMOD_TIMEUNIT_PCMBYTES,
    /// Raw file bytes of (compressed) sound data (does not include headers).
    ///
    /// Only used by [`Sound::get_length`] and [`Channel::get_position`].
    RawBytes = FMOD_TIMEUNIT_RAWBYTES,
    /// Fractions of 1 PCM sample. Unsigned int range 0 to `0xFFFFFFFF`.
    ///
    /// Used for sub-sample granularity for DSP purposes.
    PCMFraction = FMOD_TIMEUNIT_PCMFRACTION,
    /// MOD/S3M/XM/IT. Order in a sequenced module format.
    ///
    /// Use [`Sound::get_format`] to determine the PCM format being decoded to.
    ModOrder = FMOD_TIMEUNIT_MODORDER,
    /// MOD/S3M/XM/IT. Current row in a sequenced module format.
    ///
    /// Cannot use with [`Channel::set_position`].
    /// [`Sound::get_length`] will return the number of rows in the currently playing or seeked to pattern.
    ModRow = FMOD_TIMEUNIT_MODROW,
    /// MOD/S3M/XM/IT. Current pattern in a sequenced module format.
    ///
    /// Cannot use with [`Channel::set_position`].
    /// [`Sound::get_length`] will return the number of patterns in the song and [`Channel::get_position`] will return the currently playing pattern.
    ModPattern = FMOD_TIMEUNIT_MODPATTERN,
}

/// Assigns an enumeration for a speaker index.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
#[repr(i32)]
pub enum Speaker {
    /// No speaker.
    None = FMOD_SPEAKER_NONE,
    /// The front right speaker
    FrontLeft = FMOD_SPEAKER_FRONT_LEFT,
    /// The front right speaker
    FrontRight = FMOD_SPEAKER_FRONT_RIGHT,
    /// The front center speaker
    FrontCenter = FMOD_SPEAKER_FRONT_CENTER,
    /// The LFE or 'subwoofer' speaker
    LowFrequency = FMOD_SPEAKER_LOW_FREQUENCY,
    /// The surround left (usually to the side) speaker
    SurroundLeft = FMOD_SPEAKER_SURROUND_LEFT,
    /// The surround right (usually to the side) speaker
    SurroundRight = FMOD_SPEAKER_SURROUND_RIGHT,
    /// The back left speaker
    BackLeft = FMOD_SPEAKER_BACK_LEFT,
    /// The back right speaker
    BackRight = FMOD_SPEAKER_BACK_RIGHT,
    /// The top front left speaker
    TopFrontLeft = FMOD_SPEAKER_TOP_FRONT_LEFT,
    /// The top front right speaker
    TopFrontRight = FMOD_SPEAKER_TOP_FRONT_RIGHT,
    /// The top back left speaker
    TopBackLeft = FMOD_SPEAKER_TOP_BACK_LEFT,
    /// The top back right speaker
    TopBackRight = FMOD_SPEAKER_TOP_BACK_RIGHT,
}

/// Types of plug-in used to extend functionality.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum PluginType {
    /// Audio output interface plug-in represented with [`FMOD_OUTPUT_DESCRIPTION`].
    Output = FMOD_PLUGINTYPE_OUTPUT,
    /// File format codec plug-in represented with [`FMOD_CODEC_DESCRIPTION`].
    Codec = FMOD_PLUGINTYPE_CODEC,
    /// DSP unit plug-in represented with [`FMOD_DSP_DESCRIPTION`].
    DSP = FMOD_PLUGINTYPE_DSP,
}

/// DSP types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum DspType {
    /// Was created via a non-FMOD plug-in and has an unknown purpose.
    Unknown = FMOD_DSP_TYPE_UNKNOWN,
    /// Does not process the signal. Acts as a unit purely for mixing inputs.
    Mixer = FMOD_DSP_TYPE_MIXER,
    /// Generates sine/square/saw/triangle or noise tones.
    ///
    /// See [`FMOD_DSP_OSCILLATOR`] for parameter information.
    Oscillator = FMOD_DSP_TYPE_OSCILLATOR,
    /// Filters sound using a high quality, resonant lowpass filter algorithm but consumes more CPU time.
    ///
    /// Deprecated and will be removed in a future release.
    /// See [`FMOD_DSP_LOWPASS`] remarks for parameter information.
    #[deprecated]
    Lowpass = FMOD_DSP_TYPE_LOWPASS,
    /// Filters sound using a resonant lowpass filter algorithm that is used in Impulse Tracker,
    /// but with limited cutoff range (0 to 8060hz).
    ///
    /// See [`FMOD_DSP_ITLOWPASS`] for parameter information.
    ItLowpass = FMOD_DSP_TYPE_ITLOWPASS,
    /// Filters sound using a resonant highpass filter algorithm.
    ///
    /// Deprecated and will be removed in a future release.
    /// See [`FMOD_DSP_HIGHPASS`] remarks for parameter information.
    #[deprecated]
    Highpass = FMOD_DSP_TYPE_HIGHPASS,
    /// Produces an echo on the sound and fades out at the desired rate.
    ///
    /// See [`FMOD_DSP_ECHO`] for parameter information.
    Echo = FMOD_DSP_TYPE_ECHO,
    /// Pans and scales the volume of a unit.
    ///
    /// See [`FMOD_DSP_FADER`] for parameter information.
    Fader = FMOD_DSP_TYPE_FADER,
    /// Produces a flange effect on the sound.
    ///
    /// See [`FMOD_DSP_FLANGE`] for parameter information.
    Flange = FMOD_DSP_TYPE_FLANGE,
    /// Distorts the sound.
    ///
    /// See [`FMOD_DSP_DISTORTION`] for parameter information.
    Distortion = FMOD_DSP_TYPE_DISTORTION,
    /// Normalizes or amplifies the sound to a certain level.
    ///
    /// See [`FMOD_DSP_NORMALIZE`] for parameter information.
    Normalize = FMOD_DSP_TYPE_NORMALIZE,
    /// Limits the sound to a certain level.
    ///
    /// See [`FMOD_DSP_LIMITER`] for parameter information.
    Limiter = FMOD_DSP_TYPE_LIMITER,
    /// Attenuates or amplifies a selected frequency range.
    ///
    /// Deprecated and will be removed in a future release.
    /// See [`FMOD_DSP_PARAMEQ`] for parameter information.
    #[deprecated]
    ParamEq = FMOD_DSP_TYPE_PARAMEQ,
    /// Bends the pitch of a sound without changing the speed of playback.
    ///
    /// See [`FMOD_DSP_PITCHSHIFT`] for parameter information.
    PitchShift = FMOD_DSP_TYPE_PITCHSHIFT,
    /// Produces a chorus effect on the sound.
    ///
    /// See [`FMOD_DSP_CHORUS`] for parameter information.
    Chorus = FMOD_DSP_TYPE_CHORUS,
    #[cfg(fmod_eq_2_2)]
    VstPlugin = FMOD_DSP_TYPE_VSTPLUGIN,
    #[cfg(fmod_eq_2_2)]
    WinampPlugin = FMOD_DSP_TYPE_WINAMPPLUGIN,
    /// Produces an echo on the sound and fades out at the desired rate as is used in Impulse Tracker.
    ///
    /// See [`FMOD_DSP_ITECHO`] for parameter information.
    ItEcho = FMOD_DSP_TYPE_ITECHO,
    /// Dynamic compression (linked/unlinked multi-channel, wideband).
    ///
    /// See [`FMOD_DSP_COMPRESSOR`] for parameter information.
    Compressor = FMOD_DSP_TYPE_COMPRESSOR,
    /// I3DL2 reverb effect.
    ///
    /// See [`FMOD_DSP_SFXREVERB`] for parameter information.
    SfxReverb = FMOD_DSP_TYPE_SFXREVERB,
    /// Filters sound using a simple lowpass with no resonance.
    ///
    /// Deprecated and will be removed in a future release.
    /// See [`FMOD_DSP_LOWPASS_SIMPLE`] remarks for parameter information.
    #[deprecated]
    LowpassSimple = FMOD_DSP_TYPE_LOWPASS_SIMPLE,
    /// Produces different delays on individual channels of the sound.
    ///
    /// See [`FMOD_DSP_DELAY`] for parameter information.
    Delay = FMOD_DSP_TYPE_DELAY,
    /// Produces a tremolo / chopper effect on the sound.
    ///
    /// See [`FMOD_DSP_TREMOLO`] for parameter information.
    Tremolo = FMOD_DSP_TYPE_TREMOLO,
    #[cfg(fmod_eq_2_2)]
    LadspaPlugin = FMOD_DSP_TYPE_LADSPAPLUGIN,
    /// Sends a copy of the signal to a return DSP anywhere in the DSP tree.
    ///
    /// See [`FMOD_DSP_SEND`] for parameter information.
    Send = FMOD_DSP_TYPE_SEND,
    /// Receives signals from a number of send DSPs.
    ///
    /// See [`FMOD_DSP_RETURN`] for parameter information.
    Return = FMOD_DSP_TYPE_RETURN,
    /// Filters sound using a simple highpass with no resonance, but has flexible cutoff and is fast.
    ///
    /// Deprecated and will be removed in a future release.
    /// See [`FMOD_DSP_HIGHPASS_SIMPLE`] remarks for parameter information.
    #[deprecated]
    HighpassSimple = FMOD_DSP_TYPE_HIGHPASS_SIMPLE,
    /// Pans the signal in 2D or 3D, possibly upmixing or downmixing as well.
    ///
    /// See [`FMOD_DSP_PAN`] for parameter information.
    Pan = FMOD_DSP_TYPE_PAN,
    /// Three-band equalizer.
    ///
    /// See [`FMOD_DSP_THREE_EQ`] for parameter information.
    ThreeEq = FMOD_DSP_TYPE_THREE_EQ,
    /// Analyzes the signal and provides spectrum information back through [`Dsp::get_parameter`].
    ///
    /// See [`FMOD_DSP_FFT`] for parameter information.
    Fft = FMOD_DSP_TYPE_FFT,
    /// Analyzes the loudness and true peak of the signal.
    LoudnessMeter = FMOD_DSP_TYPE_LOUDNESS_METER,
    #[cfg(fmod_eq_2_2)]
    EnvelopeFollower = FMOD_DSP_TYPE_ENVELOPEFOLLOWER,
    /// Convolution reverb.
    ///
    /// See [`FMOD_DSP_CONVOLUTION_REVERB`] for parameter information.
    ConvolutionReverb = FMOD_DSP_TYPE_CONVOLUTIONREVERB,
    /// Provides per channel gain,
    /// channel grouping of the input signal which also sets the speaker format for the output signal,
    /// and customizable input to output channel routing.
    ///
    /// See [`FMOD_DSP_CHANNELMIX`] for parameter information.
    ChannelMix = FMOD_DSP_TYPE_CHANNELMIX,
    /// 'sends' and 'receives' from a selection of up to 32 different slots.
    /// It is like a send/return but it uses global slots rather than returns as the destination.
    /// It also has other features.
    /// Multiple transceivers can receive from a single channel,
    /// or multiple transceivers can send to a single channel, or a combination of both.
    ///
    /// See [`FMOD_DSP_TRANSCEIVER`] for parameter information.
    Transceiver = FMOD_DSP_TYPE_TRANSCEIVER,
    /// Spatializes input signal by passing it to an external object mixer.
    ///
    /// See [`FMOD_DSP_OBJECTPAN`] for parameter information.
    ObjectPan = FMOD_DSP_TYPE_OBJECTPAN,
    /// Five band parametric equalizer.
    ///
    /// See [`FMOD_DSP_MULTIBAND_EQ`] for parameter information.
    MultibandEq = FMOD_DSP_TYPE_MULTIBAND_EQ,
    /// Three-band compressor/expander.
    ///
    /// See [`FMOD_DSP_MULTIBAND_DYNAMICS`] for parameter information.
    #[cfg(fmod_eq_2_3)]
    MultibandDynamics = FMOD_DSP_TYPE_MULTIBAND_DYNAMICS,
}

/// Port types available for routing audio.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum PortType {
    /// Background music, pass [`FMOD_PORT_INDEX_NONE`] as port index.
    Music = FMOD_PORT_TYPE_MUSIC,
    /// Copyright background music, pass [`FMOD_PORT_INDEX_NONE`] as port index.
    CopyrightMusic = FMOD_PORT_TYPE_COPYRIGHT_MUSIC,
    /// Voice chat, pass platform specific user ID of desired user as port index.
    Voice = FMOD_PORT_TYPE_VOICE,
    /// Controller speaker, pass platform specific user ID of desired user as port index.
    Controller = FMOD_PORT_TYPE_CONTROLLER,
    /// Personal audio device, pass platform specific user ID of desired user as port index.
    Personal = FMOD_PORT_TYPE_PERSONAL,
    /// Controller vibration, pass platform specific user ID of desired user as port index.
    Vibration = FMOD_PORT_TYPE_VIBRATION,
    /// Auxiliary output port, pass [`FMOD_PORT_INDEX_NONE`] as port index.
    AUX = FMOD_PORT_TYPE_AUX,
    /// Passthrough output port, pass [`FMOD_PORT_INDEX_NONE`] as port index.
    #[cfg(fmod_eq_2_3)]
    Passthrough = FMOD_PORT_TYPE_PASSTHROUGH,
    /// VR Controller vibration, pass platform specific user ID of desired user as port index.
    #[cfg(fmod_eq_2_3)]
    VrVibration = FMOD_PORT_TYPE_VR_VIBRATION,
}

/// Values specifying behavior when a sound group's max audible value is exceeded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum SoundGroupBehavior {
    /// Excess sounds will fail when calling [`System::play_sound`].
    Fail = FMOD_SOUNDGROUP_BEHAVIOR_FAIL,
    /// Excess sounds will begin mute and will become audible when sufficient sounds are stopped.
    Mute = FMOD_SOUNDGROUP_BEHAVIOR_MUTE,
    /// Excess sounds will steal from the quietest [`Sound`] playing in the group.
    StealLowest = FMOD_SOUNDGROUP_BEHAVIOR_STEALLOWEST,
}

/// List of connection types between two DSP units.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum DspConnectionType {
    /// Default connection type. Audio is mixed from the input to the output DSP's audible buffer.
    Standard = FMOD_DSPCONNECTION_TYPE_STANDARD,
    /// Sidechain connection type. Audio is mixed from the input to the output DSP's sidechain buffer.
    Sidechain = FMOD_DSPCONNECTION_TYPE_SIDECHAIN,
    /// Send connection type.
    ///
    /// Audio is mixed from the input to the output DSP's audible buffer,
    /// but the input is not executed, only copied from.
    ///
    /// A standard connection or sidechain needs to make an input execute to generate data.
    Send = FMOD_DSPCONNECTION_TYPE_SEND,
    /// Send sidechain connection type.
    ///
    /// Audio is mixed from the input to the output DSP's sidechain buffer,
    /// but the input is not executed, only copied from.
    ///
    /// A standard connection or sidechain needs to make an input execute to generate data.
    SendSidechain = FMOD_DSPCONNECTION_TYPE_SEND_SIDECHAIN,
}

/// Data parameter types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(num_enum::FromPrimitive, num_enum::IntoPrimitive)]
#[repr(i32)]
pub enum DspParameterDataType {
    /// Data type for [`FMOD_DSP_PARAMETER_OVERALLGAIN`] parameters.
    ///
    /// There should be a maximum of one per DSP.
    OverAlign = FMOD_DSP_PARAMETER_DATA_TYPE_OVERALLGAIN,
    /// Data type for [`FMOD_DSP_PARAMETER_3DATTRIBUTES`] parameters.
    ///
    /// There should be a maximum of one per DSP.
    Attributes3D = FMOD_DSP_PARAMETER_DATA_TYPE_3DATTRIBUTES,
    /// Data type for [`FMOD_DSP_PARAMETER_SIDECHAIN`] parameters.
    ///
    /// There should be a maximum of one per DSP.
    Sidechain = FMOD_DSP_PARAMETER_DATA_TYPE_SIDECHAIN,
    /// Data type for [`FMOD_DSP_PARAMETER_FFT`] parameters.
    ///
    /// There should be a maximum of one per DSP.
    FFT = FMOD_DSP_PARAMETER_DATA_TYPE_FFT,
    /// Data type for [`FMOD_DSP_PARAMETER_3DATTRIBUTES_MULTI`] parameters.
    ///
    /// There should be a maximum of one per DSP.
    Attributes3DMulti = FMOD_DSP_PARAMETER_DATA_TYPE_3DATTRIBUTES_MULTI,
    /// Data type for [`FMOD_DSP_PARAMETER_ATTENUATION_RANGE`] parameters.
    ///
    /// There should be a maximum of one per DSP.
    AttenuationRange = FMOD_DSP_PARAMETER_DATA_TYPE_ATTENUATION_RANGE,
    /// Data type for [`FMOD_DSP_PARAMETER_DYNAMIC_RESPONSE`] parameters.
    ///
    /// There should be a maximum of one per DSP.
    #[cfg(fmod_2_3)]
    DynamicResponse = FMOD_DSP_PARAMETER_DATA_TYPE_DYNAMIC_RESPONSE,
    #[num_enum(catch_all)]
    /// Default data type. All user data types should be 0 or above.
    User(i32) = FMOD_DSP_PARAMETER_DATA_TYPE_USER, // unsure if this is correct
}

/// Recognized audio formats that can be loaded into a Sound.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum SoundType {
    /// Unknown or custom codec plug-in.
    Unknown = FMOD_SOUND_TYPE_UNKNOWN,
    /// Audio Interchange File Format (.aif, .aiff).
    /// Uncompressed integer formats only.
    AIFF = FMOD_SOUND_TYPE_AIFF,
    /// Microsoft Advanced Systems Format (.asf, .wma, .wmv).
    /// Platform provided decoder, available only on Windows.
    ASF = FMOD_SOUND_TYPE_ASF,
    /// Downloadable Sound (.dls).
    /// Multi-sound bank format used by MIDI (.mid).
    DLS = FMOD_SOUND_TYPE_DLS,
    /// Free Lossless Audio Codec (.flac).
    FLAC = FMOD_SOUND_TYPE_FLAC,
    /// FMOD Sample Bank (.fsb).
    /// Proprietary multi-sound bank format.
    /// Supported encodings: PCM16, FADPCM, Vorbis, AT9, XMA, Opus.
    FSB = FMOD_SOUND_TYPE_FSB,
    /// Impulse Tracker (.it).
    IT = FMOD_SOUND_TYPE_IT,
    /// Musical Instrument Digital Interface (.mid).
    MIDI = FMOD_SOUND_TYPE_MIDI,
    /// Protracker / Fasttracker Module File (.mod).
    MOD = FMOD_SOUND_TYPE_MOD,
    /// Moving Picture Experts Group (.mp2, .mp3).
    /// Also supports .wav (RIFF) container format.
    MPEG = FMOD_SOUND_TYPE_MPEG,
    /// Ogg Vorbis (.ogg).
    OGGVORBIS = FMOD_SOUND_TYPE_OGGVORBIS,
    /// Play list information container (.asx, .pls, .m3u, .wax).
    /// No audio, tags only.
    Playlist = FMOD_SOUND_TYPE_PLAYLIST,
    /// Raw uncompressed PCM data (.raw).
    RAW = FMOD_SOUND_TYPE_RAW,
    /// `ScreamTracker` 3 Module (.s3m).
    S3M = FMOD_SOUND_TYPE_S3M,
    /// User created sound.
    User = FMOD_SOUND_TYPE_USER,
    /// Microsoft Waveform Audio File Format (.wav).
    /// Supported encodings: Uncompressed PCM, IMA ADPCM.
    /// Platform provided ACM decoder extensions, available only on Windows.
    WAV = FMOD_SOUND_TYPE_WAV,
    /// `FastTracker` 2 Extended Module (.xm).
    XM = FMOD_SOUND_TYPE_XM,
    ///     Microsoft XMA bit-stream supported by FSB (.fsb) container format.
    /// Platform provided decoder, available only on Xbox.
    XMA = FMOD_SOUND_TYPE_XMA,
    /// Apple Audio Queue decoder (.mp4, .m4a, .mp3).
    /// Supported encodings: AAC, ALAC, MP3.
    /// Platform provided decoder, available only on iOS / tvOS devices.
    AudioQueue = FMOD_SOUND_TYPE_AUDIOQUEUE,
    /// Sony ATRAC9 bit-stream supported by FSB (.fsb) container format.
    /// Platform provided decoder, available only on `PlayStation`.
    AT9 = FMOD_SOUND_TYPE_AT9,
    /// Vorbis bit-stream supported by FSB (.fsb) container format.
    Vorbis = FMOD_SOUND_TYPE_VORBIS,
    /// Microsoft Media Foundation decoder (.asf, .wma, .wmv, .mp4, .m4a).
    /// Platform provided decoder, available only on UWP.
    MediaFoundation = FMOD_SOUND_TYPE_MEDIA_FOUNDATION,
    /// Google Media Codec decoder (.m4a, .mp4).
    /// Platform provided decoder, available only on Android.
    MediaCodec = FMOD_SOUND_TYPE_MEDIACODEC,
    /// FMOD Adaptive Differential Pulse Code Modulation bit-stream supported by FSB (.fsb) container format.
    FADPCM = FMOD_SOUND_TYPE_FADPCM,
    /// Opus bit-stream supported by FSB (.fsb) container format.
    /// Platform provided decoder, available only on Xbox Series X|S, PS5, and Switch.
    OPUS = FMOD_SOUND_TYPE_OPUS,
}

/// These definitions describe the native format of the hardware or software buffer that will be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum SoundFormat {
    /// Uninitalized / unknown.
    None = FMOD_SOUND_FORMAT_NONE,
    /// 8bit integer PCM data.
    PCM8 = FMOD_SOUND_FORMAT_PCM8,
    /// 16bit integer PCM data.
    PCM16 = FMOD_SOUND_FORMAT_PCM16,
    /// 24bit integer PCM data.
    PCM24 = FMOD_SOUND_FORMAT_PCM24,
    /// 32bit integer PCM data.
    PCM32 = FMOD_SOUND_FORMAT_PCM32,
    /// 32bit floating point PCM data.
    PCMFloat = FMOD_SOUND_FORMAT_PCMFLOAT,
    /// Sound data is in its native compressed format.
    ///
    /// See [`FMOD_CREATECOMPRESSEDSAMPLE`]
    BitStream = FMOD_SOUND_FORMAT_BITSTREAM,
}

/// List of tag data / metadata types that could be stored within a sound. These include id3 tags, metadata from netstreams and vorbis/asf data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum TagType {
    /// Tag type that is not recognized by FMOD
    Unknown = FMOD_TAGTYPE_UNKNOWN,
    /// MP3 ID3 Tag 1.0. Typically 1 tag stored 128 bytes from end of an MP3 file.
    ID3V1 = FMOD_TAGTYPE_ID3V1,
    /// MP3 ID3 Tag 2.0. Variable length tags with more than 1 possible.
    ID3V2 = FMOD_TAGTYPE_ID3V2,
    /// Metadata container used in Vorbis, FLAC, Theora, Speex and Opus file formats.
    VorbisComment = FMOD_TAGTYPE_VORBISCOMMENT,
    /// `SHOUTcast` internet stream metadata which can be issued during playback.
    ShoutCast = FMOD_TAGTYPE_SHOUTCAST,
    /// Icecast internet stream metadata which can be issued during playback.
    IceCast = FMOD_TAGTYPE_ICECAST,
    /// Advanced Systems Format metadata typically associated with Windows Media formats such as WMA.
    ASF = FMOD_TAGTYPE_ASF,
    /// Metadata stored inside a MIDI file.
    MIDI = FMOD_TAGTYPE_MIDI,
    /// Playlist files such as PLS,M3U,ASX and WAX will populate playlist information through this tag type.
    Playlist = FMOD_TAGTYPE_PLAYLIST,
    /// Tag type used by FMOD's MIDI, MOD, S3M, XM, IT format support, and netstreams to notify of internet stream events like a sample rate change.
    Fmod = FMOD_TAGTYPE_FMOD,
    /// For codec developers, this tag type can be used with [`FMOD_CODEC_METADATA_FUNC`] to generate custom metadata.
    User = FMOD_TAGTYPE_USER,
}

/// These values describe what state a sound is in after [`FMOD_NONBLOCKING`] has been used to open it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum OpenState {
    /// Opened and ready to play.
    Ready = FMOD_OPENSTATE_READY,
    /// Initial load in progress.
    Loading = FMOD_OPENSTATE_LOADING,
    /// Failed to open - file not found, out of memory etc.
    ///
    /// See return value of [`Sound::get_open_state`]for what happened.
    Error(Error) = FMOD_OPENSTATE_ERROR,
    /// Connecting to remote host (internet sounds only).
    Connecting = FMOD_OPENSTATE_CONNECTING,
    /// Buffering data.
    Buffering = FMOD_OPENSTATE_BUFFERING,
    /// Seeking to subsound and re-flushing stream buffer.
    Seeking = FMOD_OPENSTATE_SEEKING,
    /// Ready and playing, but not possible to release at this time without stalling the main thread.
    Playing = FMOD_OPENSTATE_PLAYING,
    /// Seeking within a stream to a different position.
    SetPosition = FMOD_OPENSTATE_SETPOSITION,
}

impl OpenState {
    /// Try creating a `OpenState` from its FFI equivalent.
    pub fn try_from_ffi(value: FMOD_OPENSTATE, error: Option<Error>) -> Result<Self> {
        match value {
            FMOD_OPENSTATE_READY => Ok(OpenState::Ready),
            FMOD_OPENSTATE_LOADING => Ok(OpenState::Loading),
            FMOD_OPENSTATE_ERROR => error.map(OpenState::Error).ok_or(Error::InvalidParam),
            FMOD_OPENSTATE_CONNECTING => Ok(OpenState::Connecting),
            FMOD_OPENSTATE_BUFFERING => Ok(OpenState::Buffering),
            FMOD_OPENSTATE_SEEKING => Ok(OpenState::Seeking),
            FMOD_OPENSTATE_PLAYING => Ok(OpenState::Playing),
            FMOD_OPENSTATE_SETPOSITION => Ok(OpenState::SetPosition),
            _ => Err(Error::EnumFromPrivitive {
                name: "LoadingState",
                primitive: i64::from(value),
            }),
        }
    }
}

/// Speaker ordering for multi-channel signals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum ChannelOrder {
    /// Left, Right, Center, LFE, Surround Left, Surround Right, Back Left, Back Right (see [`Speaker`] enumeration)
    Default = FMOD_CHANNELORDER_DEFAULT,
    /// Left, Right, Center, LFE, Back Left, Back Right, Surround Left, Surround Right (as per Microsoft .wav WAVEFORMAT structure master order)
    WaveFormat = FMOD_CHANNELORDER_WAVEFORMAT,
    /// Left, Center, Right, Surround Left, Surround Right, LFE
    ProTools = FMOD_CHANNELORDER_PROTOOLS,
    /// Mono, Mono, Mono, Mono, Mono, Mono, ... (each channel up to [`FMOD_MAX_CHANNEL_WIDTH`] treated as mono)
    AllMono = FMOD_CHANNELORDER_ALLMONO,
    /// Left, Right, Left, Right, Left, Right, ... (each pair of channels up to [`FMOD_MAX_CHANNEL_WIDTH`] treated as stereo)
    AllStereo = FMOD_CHANNELORDER_ALLSTEREO,
    /// Left, Right, Surround Left, Surround Right, Center, LFE (as per Linux ALSA channel order)
    Alsa = FMOD_CHANNELORDER_ALSA,
}

/// List of interpolation types used for resampling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum Resampler {
    /// Default interpolation method, same as [`Resampler::Linear`].
    #[default]
    Default = FMOD_DSP_RESAMPLER_DEFAULT,
    /// No interpolation.
    /// High frequency aliasing hiss will be audible depending on the sample rate of the sound.
    NoInterp = FMOD_DSP_RESAMPLER_NOINTERP,
    /// Linear interpolation (default method).
    /// Fast and good quality, causes very slight lowpass effect on low frequency sounds.
    Linear = FMOD_DSP_RESAMPLER_LINEAR,
    /// Cubic interpolation.
    /// Slower than linear interpolation but better quality.
    Cubic = FMOD_DSP_RESAMPLER_CUBIC,
    /// 5 point spline interpolation.
    /// Slowest resampling method but best quality.
    Spline = FMOD_DSP_RESAMPLER_SPLINE,
}

/// DSP float parameter mappings.
///
/// These determine how values are mapped across dials and automation curves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[derive(
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    num_enum::UnsafeFromPrimitive
)]
// stupid enum repr hack
#[cfg_attr(target_env = "msvc", repr(i32))]
#[cfg_attr(not(target_env = "msvc"), repr(u32))]
pub enum FloatMappingType {
    /// Values mapped linearly across range.
    #[default]
    Linear = FMOD_DSP_PARAMETER_FLOAT_MAPPING_TYPE_LINEAR,
    /// A mapping is automatically chosen based on range and units.
    Auto = FMOD_DSP_PARAMETER_FLOAT_MAPPING_TYPE_AUTO,
    /// Values mapped in a piecewise linear fashion defined by [`FMOD_DSP_PARAMETER_FLOAT_MAPPING_PIECEWISE_LINEAR`].
    PiecewiceLinear = FMOD_DSP_PARAMETER_FLOAT_MAPPING_TYPE_PIECEWISE_LINEAR,
}
