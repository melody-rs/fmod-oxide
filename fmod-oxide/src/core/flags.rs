// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

bitflags::bitflags! {
  /// Configuration flags used when initializing the System object.
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  pub struct InitFlags: FMOD_INITFLAGS {
    /// Initialize normally
    const NORMAL =                  FMOD_INIT_NORMAL;
    /// No stream thread is created internally.
    /// Streams are driven from [`System::update`]. Mainly used with non-realtime outputs.
    const STREAM_FROM_UPDATE =      FMOD_INIT_STREAM_FROM_UPDATE;
    /// No mixer thread is created internally.
    /// Mixing is driven from [`System::update`].
    /// Only applies to polling based output modes such as [`FMOD_OUTPUTTYPE_NOSOUND`], [`FMOD_OUTPUTTYPE_WAVWRITER`].
    const MIX_FROM_UPDATE =         FMOD_INIT_MIX_FROM_UPDATE;
    /// 3D calculations will be performed in right-handed coordinates, instead of the default of left-handed coordinates.
    /// See the Handedness section of the Glossary for more information.
    const RIGHTHANDED_3D =          FMOD_INIT_3D_RIGHTHANDED;
    /// Enables hard clipping of output values greater than 1.0f or less than -1.0f.
    const CLIP_OUTPUT =             FMOD_INIT_CLIP_OUTPUT;
    /// Enables usage of [`ChannelControl::setLowPassGain`],
    /// [`ChannelControl::set3DOcclusion`], or automatic usage by the Geometry API.
    ///
    /// All voices will add a software lowpass filter effect into the DSP chain which is idle unless one of the previous
    /// functions/features are used.
    const CHANNEL_LOWPASS =         FMOD_INIT_CHANNEL_LOWPASS;
    /// All [`FMOD_3D`] based voices add a software low pass and highpass filter effect into the DSP chain,
    /// which acts as a distance-automated bandpass filter.
    ///
    /// Use [`System::setAdvancedSettings`] to adjust the center frequency.
    const CHANNEL_DISTANCE_FILTER = FMOD_INIT_CHANNEL_DISTANCEFILTER;
    /// Enable TCP/IP based host which allows FMOD Studio or FMOD Profiler to connect to it,
    /// and view memory, CPU and the DSP graph in real-time.
    const PROFILE_ENABLE =          FMOD_INIT_PROFILE_ENABLE;
    /// Any sounds that are 0 volume will go virtual and not be processed except for having their positions updated virtually.
    ///
    /// Use [`System::setAdvancedSettings`] to adjust what volume besides zero to switch to virtual at.
    const VOL_0_BECOMES_VIRTUAL =   FMOD_INIT_VOL0_BECOMES_VIRTUAL;
    /// With the geometry engine, only process the closest polygon rather than accumulating all polygons the sound to listener line intersects.
    const GEOMETRY_USE_CLOSEST =    FMOD_INIT_GEOMETRY_USECLOSEST;
    /// When using [`FMOD_SPEAKERMODE_5POINT1`] with a stereo output device,
    /// use the Dolby Pro Logic II downmix algorithm instead of the default stereo downmix algorithm.
    const PREFER_DOLBY_DOWNMIX =    FMOD_INIT_PREFER_DOLBY_DOWNMIX;
    /// This flag cannot be used normally as this crate has guardrails preventing it.
    /// It is still here for completeness' sake, though.
    const THREAD_UNSAFE =           FMOD_INIT_THREAD_UNSAFE;
    /// Slower, but adds level metering for every single DSP unit in the graph.
    /// Use [`DSP::setMeteringEnabled`] to turn meters off individually.
    ///
    /// Setting this flag implies [`PROFILE_ENABLE`].
    const PROFILE_METER_ALL =       FMOD_INIT_PROFILE_METER_ALL;
    /// Enables memory allocation tracking.
    /// Currently this is only useful when using the Studio API.
    /// Increases memory footprint and reduces performance.
    ///
    /// This flag is implied by [`FMOD_STUDIO_INIT_MEMORY_TRACKING`].
    const MEMORY_TRACKING =         FMOD_INIT_MEMORY_TRACKING;
  }
}

impl From<FMOD_INITFLAGS> for InitFlags {
    fn from(value: FMOD_INITFLAGS) -> Self {
        InitFlags::from_bits_truncate(value)
    }
}

impl From<InitFlags> for FMOD_INITFLAGS {
    fn from(value: InitFlags) -> Self {
        value.bits()
    }
}

bitflags::bitflags! {
  /// Bitfield for specifying the CPU core a given thread runs on.
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  pub struct ThreadAffinity: FMOD_THREAD_AFFINITY {
    /// For a given thread use the default listed below, i.e. [`FMOD_THREAD_TYPE_MIXER`] uses [`FMOD_THREAD_AFFINITY_MIXER`].
    const GROUP_DEFAULT      = FMOD_THREAD_AFFINITY_GROUP_DEFAULT       as FMOD_THREAD_AFFINITY;
    /// Grouping A is recommended to isolate the mixer thread [`FMOD_THREAD_TYPE_MIXER`].
    const GROUP_A            = FMOD_THREAD_AFFINITY_GROUP_A             as FMOD_THREAD_AFFINITY;
    /// Grouping B is recommended to isolate the Studio update thread [`FMOD_THREAD_TYPE_STUDIO_UPDATE`].
    const GROUP_B            = FMOD_THREAD_AFFINITY_GROUP_B             as FMOD_THREAD_AFFINITY;
    /// Grouping C is recommended for all remaining threads.
    const GROUP_C            = FMOD_THREAD_AFFINITY_GROUP_C             as FMOD_THREAD_AFFINITY;
    /// Default affinity for [`FMOD_THREAD_TYPE_MIXER`].
    const MIXER              = FMOD_THREAD_AFFINITY_MIXER               as FMOD_THREAD_AFFINITY;
    /// Default affinity for [`FMOD_THREAD_TYPE_FEEDER`].
    const FEEDER             = FMOD_THREAD_AFFINITY_FEEDER              as FMOD_THREAD_AFFINITY;
    /// Default affinity for [`FMOD_THREAD_TYPE_STREAM`].
    const STREAM             = FMOD_THREAD_AFFINITY_STREAM              as FMOD_THREAD_AFFINITY;
    /// Default affinity for [`FMOD_THREAD_TYPE_FILE`].
    const FILE               = FMOD_THREAD_AFFINITY_FILE                as FMOD_THREAD_AFFINITY;
    /// Default affinity for [`FMOD_THREAD_TYPE_NONBLOCKING`].
    const NONBLOCKING        = FMOD_THREAD_AFFINITY_NONBLOCKING         as FMOD_THREAD_AFFINITY;
    /// Default affinity for [`FMOD_THREAD_TYPE_RECORD`].
    const RECORD             = FMOD_THREAD_AFFINITY_RECORD              as FMOD_THREAD_AFFINITY;
    /// Default affinity for [`FMOD_THREAD_TYPE_GEOMETRY`].
    const GEOMETRY           = FMOD_THREAD_AFFINITY_GEOMETRY            as FMOD_THREAD_AFFINITY;
    /// Default affinity for [`FMOD_THREAD_TYPE_PROFILER`].
    const PROFILER           = FMOD_THREAD_AFFINITY_PROFILER            as FMOD_THREAD_AFFINITY;
    /// Default affinity for [`FMOD_THREAD_TYPE_STUDIO_UPDATE`].
    const STUDIO_UPDATE      = FMOD_THREAD_AFFINITY_STUDIO_UPDATE       as FMOD_THREAD_AFFINITY;
    /// Default affinity for [`FMOD_THREAD_TYPE_STUDIO_LOAD_BANK`].
    const STUDIO_LOAD_BANK   = FMOD_THREAD_AFFINITY_STUDIO_LOAD_BANK    as FMOD_THREAD_AFFINITY;
    /// Default affinity for [`FMOD_THREAD_TYPE_STUDIO_LOAD_SAMPLE`].
    const STUDIO_LOAD_SAMPLE = FMOD_THREAD_AFFINITY_STUDIO_LOAD_SAMPLE  as FMOD_THREAD_AFFINITY;
    /// Default affinity for [`FMOD_THREAD_TYPE_CONVOLUTION1`].
    const CONVOLUTION_1      = FMOD_THREAD_AFFINITY_CONVOLUTION1  as FMOD_THREAD_AFFINITY;
        /// Default affinity for [`FMOD_THREAD_TYPE_CONVOLUTION2`].
    const CONVOLUTION_2      = FMOD_THREAD_AFFINITY_CONVOLUTION2  as FMOD_THREAD_AFFINITY;
    /// Assign to all cores.
    const CORE_ALL           = FMOD_THREAD_AFFINITY_CORE_ALL            as FMOD_THREAD_AFFINITY;
    /// Assign to core 0.
    const CORE_0             = FMOD_THREAD_AFFINITY_CORE_0              as FMOD_THREAD_AFFINITY;
    /// Assign to core 1.
    const CORE_1             = FMOD_THREAD_AFFINITY_CORE_1              as FMOD_THREAD_AFFINITY;
    /// Assign to core 2.
    const CORE_2             = FMOD_THREAD_AFFINITY_CORE_2              as FMOD_THREAD_AFFINITY;
    /// Assign to core 3.
    const CORE_3             = FMOD_THREAD_AFFINITY_CORE_3              as FMOD_THREAD_AFFINITY;
    /// Assign to core 4.
    const CORE_4             = FMOD_THREAD_AFFINITY_CORE_4              as FMOD_THREAD_AFFINITY;
    /// Assign to core 5.
    const CORE_5             = FMOD_THREAD_AFFINITY_CORE_5              as FMOD_THREAD_AFFINITY;
    /// Assign to core 6.
    const CORE_6             = FMOD_THREAD_AFFINITY_CORE_6              as FMOD_THREAD_AFFINITY;
    /// Assign to core 7.
    const CORE_7             = FMOD_THREAD_AFFINITY_CORE_7              as FMOD_THREAD_AFFINITY;
    /// Assign to core 8.
    const CORE_8             = FMOD_THREAD_AFFINITY_CORE_8              as FMOD_THREAD_AFFINITY;
    /// Assign to core 9.
    const CORE_9             = FMOD_THREAD_AFFINITY_CORE_9              as FMOD_THREAD_AFFINITY;
    /// Assign to core 10.
    const CORE_10            = FMOD_THREAD_AFFINITY_CORE_10             as FMOD_THREAD_AFFINITY;
    /// Assign to core 11.
    const CORE_11            = FMOD_THREAD_AFFINITY_CORE_11             as FMOD_THREAD_AFFINITY;
    /// Assign to core 12.
    const CORE_12            = FMOD_THREAD_AFFINITY_CORE_12             as FMOD_THREAD_AFFINITY;
    /// Assign to core 13.
    const CORE_13            = FMOD_THREAD_AFFINITY_CORE_13             as FMOD_THREAD_AFFINITY;
    /// Assign to core 14.
    const CORE_14            = FMOD_THREAD_AFFINITY_CORE_14             as FMOD_THREAD_AFFINITY;
    /// Assign to core 15.
    const CORE_15            = FMOD_THREAD_AFFINITY_CORE_15             as FMOD_THREAD_AFFINITY;
  }
}

impl From<FMOD_THREAD_AFFINITY> for ThreadAffinity {
    fn from(value: FMOD_THREAD_AFFINITY) -> Self {
        ThreadAffinity::from_bits_truncate(value)
    }
}

impl From<ThreadAffinity> for FMOD_THREAD_AFFINITY {
    fn from(value: ThreadAffinity) -> Self {
        value.bits()
    }
}

bitflags::bitflags! {
  /// [`Sound`] description bitfields.
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  pub struct Mode: FMOD_MODE {
    /// Default for all modes listed below.
    const DEFAULT                   = FMOD_DEFAULT;
    /// For non looping [`Sound`]s. (Default)
    ///
    /// Overrides [`LOOP_NORMAL`] / [`LOOP_BIDI`].
    const LOOP_OFF                  = FMOD_LOOP_OFF;
    /// For forward looping [`Sound`]s.
    const LOOP_NORMAL               = FMOD_LOOP_NORMAL;
    /// For bidirectional looping [`Sound`]s. (only works on non-streaming, real voices).
    const LOOP_BIDI                 = FMOD_LOOP_BIDI;
    /// Ignores any 3d processing. (Default)
    const D2                        = FMOD_2D;
    /// Makes the [`Sound`] positionable in 3D. Overrides [`D2`].
    const D3                        = FMOD_3D;
    /// Decompress at runtime,
    /// streaming from the source provided (ie from disk).
    /// Overrides [`FMOD_CREATESAMPLE`] and [`FMOD_CREATECOMPRESSEDSAMPLE`].
    /// Note a stream can only be played once at a time due to a stream only having 1 stream buffer and file handle.
    /// Open multiple streams to have them play concurrently.
    const CREATE_STREAM             = FMOD_CREATESTREAM;
    /// Decompress at loadtime,
    /// decompressing or decoding whole file into memory as the target sample format (ie PCM).
    /// Fastest for playback and most flexible.
    const CREATE_SAMPLE             = FMOD_CREATESAMPLE;
    /// Load MP2/MP3/FADPCM/IMAADPCM/Vorbis/AT9 or XMA into memory and leave it compressed.
    /// Vorbis/AT9/FADPCM encoding only supported in the .FSB container format.
    /// During playback the FMOD software mixer will decode it in realtime as a 'compressed sample'.
    /// Overrides [`FMOD_CREATESAMPLE`].
    /// If the sound data is not one of the supported formats,
    /// it will behave as if it was created with [`FMOD_CREATESAMPLE`] and decode the sound into PCM.
    const CREATE_COMPRESSED_SAMPLE  = FMOD_CREATECOMPRESSEDSAMPLE;
    /// Opens a user-created static sample or stream.
    const OPEN_USER                 = FMOD_OPENUSER;
    /// Opens a sound with a pointer to memory.
    /// Duplicates the pointer into its own buffer.
    const OPEN_MEMORY               = FMOD_OPENMEMORY;
    /// Opens a sound with a pointer to memory.
    const OPEN_MEMORY_POINT         = FMOD_OPENMEMORY_POINT;
    /// Will ignore file format and treat as raw pcm.
    const OPEN_RAW                  = FMOD_OPENRAW;
    /// Just open the file, don't prebuffer or read.
    /// Good for fast opens for info, or when [`Sound::readData`] is to be used.
    const OPEN_ONLY                 = FMOD_OPENONLY;
    /// For [`System::createSound`] - for accurate [`Sound::getLength`] / [`Channel::setPosition`] on VBR MP3,
    /// and MOD/S3M/XM/IT/MIDI files.
    /// Scans file first, so takes longer to open.
    /// [`FMOD_OPENONLY`] does not affect this.
    const ACCURATE_TIME             = FMOD_ACCURATETIME;
    /// For corrupted / bad MP3 files.
    /// This will search all the way through the file until it hits a valid MPEG header.
    /// Normally only searches for 4k.
    const MPEG_SEARCH               = FMOD_MPEGSEARCH;
    /// For opening [`Sound`]s and getting streamed subsounds (seeking) asynchronously.
    /// Use [`Sound::getOpenState`] to poll the state of the [`Sound`] as it opens or retrieves the subsound in the background.
    const NONBLOCKING               = FMOD_NONBLOCKING;
    /// Unique [`Sound`], can only be played one at a time.
    const UNIQUE                    = FMOD_UNIQUE;
    /// Make the [`Sound`]'s position, velocity and orientation relative to the listener.
    const HEADRELATIVE_3D           = FMOD_3D_HEADRELATIVE;
    /// Make the [`Sound`]'s position, velocity and orientation absolute (relative to the world). (Default)
    const WORLDRELATIVE_3D          = FMOD_3D_WORLDRELATIVE;
    /// This sound follows an inverse roll-off model.
    /// Below mindistance, the volume is unattenuated; as distance increases above mindistance,
    /// the volume attenuates using mindistance/distance as the gradient until it reaches maxdistance,
    /// where it stops attenuating.
    /// For this roll-off mode, distance values greater than mindistance are scaled according to the rolloffscale.
    /// This roll-off mode accurately models the way sounds attenuate over distance in the real world. (Default)
    const INVERSE_ROLLOFF_3D        = FMOD_3D_INVERSEROLLOFF;
    /// This sound follows a linear roll-off model.
    /// Below mindistance, the volume is unattenuated; as distance increases from mindistance to maxdistance,
    /// the volume attenuates to silence using a linear gradient.
    /// For this roll-off mode, distance values greater than mindistance are scaled according to the rolloffscale.
    /// While this roll-off mode is not as realistic as inverse roll-off mode, it is easier to comprehend.
    const LINEAR_ROLLOFF_3D         = FMOD_3D_LINEARROLLOFF;
    /// This sound follows a linear-square roll-off model.
    /// Below mindistance, the volume is unattenuated; as distance increases from mindistance to maxdistance,
    /// the volume attenuates to silence according to a linear squared gradient.
    /// For this roll-off mode, distance values greater than mindistance are scaled according to the rolloffscale.
    /// This roll-off mode provides steeper volume ramping close to the mindistance,
    /// and more gradual ramping close to the maxdistance, than linear roll-off mode.
    const LINEAR_SQUARE_ROLLOFF_3D  = FMOD_3D_LINEARSQUAREROLLOFF;
    /// This sound follows a combination of the inverse and linear-square roll-off models.
    /// At short distances where inverse roll-off would provide greater attenuation,
    /// it functions as inverse roll-off mode; then at greater distances where linear-square roll-off mode would provide greater attenuation,
    /// it uses that roll-off mode instead.
    /// For this roll-off mode, distance values greater than mindistance are scaled according to the rolloffscale.
    /// Inverse tapered roll-off mode approximates realistic behavior while still guaranteeing the sound attenuates to silence at maxdistance.
    const INVERSE_TAPERED_ROLLOFF_3D = FMOD_3D_INVERSETAPEREDROLLOFF;
    /// This sound follow a roll-off model defined by [`Sound::set3DCustomRolloff`] / [`ChannelControl::set3DCustomRolloff`].
    /// This roll-off mode provides greater freedom and flexibility than any other, but must be defined manually.
    const CUSTOM_ROLLOFF_3D         = FMOD_3D_CUSTOMROLLOFF;
    /// Is not affected by geometry occlusion.
    /// If not specified in [`Sound::setMode`], or [`ChannelControl::setMode`],
    /// the flag is cleared and it is affected by geometry again.
    const IGNORE_GEOMETRY_3D        = FMOD_3D_IGNOREGEOMETRY;
    /// Skips id3v2/asf/etc tag checks when opening a Sound, to reduce seek/read overhead when opening files.
    const IGNORE_TAGS               = FMOD_IGNORETAGS;
    /// Removes some features from samples to give a lower memory overhead, like [`Sound::getName`].
    const LOWMEM                    = FMOD_LOWMEM;
    /// For Channels that start virtual (due to being quiet or low importance),
    /// instead of swapping back to audible,
    /// and playing at the correct offset according to time, this flag makes the Channel play from the start.
    const VIRTUAL_PLAYFROM_START    = FMOD_VIRTUAL_PLAYFROMSTART;
  }
}

impl From<FMOD_MODE> for Mode {
    fn from(value: FMOD_MODE) -> Self {
        Mode::from_bits_truncate(value)
    }
}

impl From<Mode> for FMOD_MODE {
    fn from(value: Mode) -> Self {
        value.bits()
    }
}

bitflags::bitflags! {
  /// Flags that describe the speakers present in a given signal.
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  pub struct ChannelMask: FMOD_CHANNELMASK {
    /// Front left channel.
    const FRONT_LEFT        = FMOD_CHANNELMASK_FRONT_LEFT;
    /// Front right channel.
    const FRONT_RIGHT       = FMOD_CHANNELMASK_FRONT_RIGHT;
    /// Front center channel.
    const FRONT_CENTER      = FMOD_CHANNELMASK_FRONT_CENTER;
    /// Low frequency channel.
    const LOW_FREQUENCY     = FMOD_CHANNELMASK_LOW_FREQUENCY;
    /// Surround left channel.
    const SURROUND_LEFT     = FMOD_CHANNELMASK_SURROUND_LEFT;
    /// Surround right channel.
    const SURROUND_RIGHT    = FMOD_CHANNELMASK_SURROUND_RIGHT;
    /// Back left channel.
    const BACK_LEFT         = FMOD_CHANNELMASK_BACK_LEFT;
    /// Back right channel.
    const BACK_RIGHT        = FMOD_CHANNELMASK_BACK_RIGHT;
    /// Back center channel, not represented in any [`FMOD_SPEAKERMODE`].
    const BACK_CENTER       = FMOD_CHANNELMASK_BACK_CENTER;
    /// Mono channel mask.
    const MONO              = FMOD_CHANNELMASK_MONO;
    /// Stereo channel mask.
    const STEREO            = FMOD_CHANNELMASK_STEREO;
    /// Left / right / center channel mask.
    const LRC               = FMOD_CHANNELMASK_LRC;
    /// Quadphonic channel mask.
    const QUAD              = FMOD_CHANNELMASK_QUAD;
    /// 5.0 surround sound channel mask.
    const SURROUND          = FMOD_CHANNELMASK_SURROUND;
    /// 5.1 surround sound channel mask.
    const _5POINT1          = FMOD_CHANNELMASK_5POINT1;
    /// 5.1 surround sound channel mask, using rears instead of surrounds.
    const _5POINT1_REARS    = FMOD_CHANNELMASK_5POINT1_REARS;
    /// 7.0 surround sound channel mask.
    const _7POINT0          = FMOD_CHANNELMASK_7POINT0;
    /// 7.1 surround sound channel mask.
    const _7POINT1          = FMOD_CHANNELMASK_7POINT1;
  }
}

impl From<FMOD_CHANNELMASK> for ChannelMask {
    fn from(value: FMOD_CHANNELMASK) -> Self {
        ChannelMask::from_bits_truncate(value)
    }
}

impl From<ChannelMask> for FMOD_CHANNELMASK {
    fn from(value: ChannelMask) -> Self {
        value.bits()
    }
}

bitflags::bitflags! {
  /// Flags that provide additional information about a particular driver.
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  pub struct DriverState: FMOD_DRIVER_STATE {
    /// Device is currently plugged in.
    const CONNECTED = FMOD_DRIVER_STATE_CONNECTED;
    /// Device is the users preferred choice.
    const DEFAULT   = FMOD_DRIVER_STATE_DEFAULT;
  }
}

impl From<FMOD_DRIVER_STATE> for DriverState {
    fn from(value: FMOD_DRIVER_STATE) -> Self {
        DriverState::from_bits_truncate(value)
    }
}

impl From<DriverState> for FMOD_DRIVER_STATE {
    fn from(value: DriverState) -> Self {
        value.bits()
    }
}
