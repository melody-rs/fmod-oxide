// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{FmodResultExt, Result};
use crate::{InitFlags, OutputType, SpeakerMode, System};
use fmod_sys::*;
use std::ffi::{c_int, c_uint, c_void};

/// A builder for creating and initializing a [`System`].
///
/// Handles setting values that can only be set before initialization for you.
#[derive(Debug)]
pub struct SystemBuilder {
    pub(crate) system: *mut FMOD_SYSTEM,
    pub(crate) thread_unsafe: bool,
}

#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for SystemBuilder {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for SystemBuilder {}

#[cfg(doc)]
use crate::{debug, memory};

impl SystemBuilder {
    /// Creates a new [`SystemBuilder`].
    ///
    /// # Safety
    ///
    /// This must be called first to create an FMOD System object before any other API calls (except for [`memory::initialize`] and [`debug::initialize`]).
    /// Use this function to create 1 or multiple instances of FMOD System objects.
    ///
    /// Calls to [`SystemBuilder::new`] and [`System::release`] are not thread-safe.
    /// Do not call these functions simultaneously from multiple threads at once.
    pub unsafe fn new() -> Result<Self> {
        let mut system = std::ptr::null_mut();
        unsafe { FMOD_System_Create(&raw mut system, FMOD_VERSION).to_result()? };

        Ok(SystemBuilder {
            system,
            thread_unsafe: false,
        })
    }

    /// # Safety
    ///
    /// This function intializes FMOD to be thread unsafe, which makes *EVERY* Struct in this crate `!Send` and `!Sync` *without* marking them as `!Send` and `!Sync`.
    /// This means that there are no handrails preventing you from using FMOD across multiple threads, and you *must* ensure this yourself!
    #[cfg(not(feature = "thread-unsafe"))]
    pub unsafe fn thread_unsafe(&mut self) {
        self.thread_unsafe = true;
    }

    #[cfg(feature = "thread-unsafe")]
    pub fn thread_unsafe(&mut self) {
        self.thread_unsafe = true;
    }

    /// Sets the output format for the software mixer.
    ///
    /// If loading banks made in FMOD Studio, this must be called with speakermode corresponding to the project output format
    /// if there is a possibility of the output audio device not matching the project format.
    /// Any differences between the project format and speakermode will cause the mix to sound wrong.
    ///
    /// By default speakermode will assume the setup the OS / output prefers.
    ///
    /// Altering the samplerate from the OS / output preferred rate may incur extra latency.
    /// Altering the speakermode from the OS / output preferred mode may cause an upmix/downmix which can alter the sound.
    ///
    /// On lower power platforms such as mobile samplerate will default to 24 kHz to reduce CPU cost.
    pub fn software_format(
        &mut self,
        sample_rate: c_int,
        speaker_mode: SpeakerMode,
        raw_speakers: c_int,
    ) -> Result<&mut Self> {
        unsafe {
            FMOD_System_SetSoftwareFormat(
                self.system,
                sample_rate,
                speaker_mode.into(),
                raw_speakers,
            )
            .to_result()?;
        };
        Ok(self)
    }

    /// Sets the output format for the software mixer.
    ///
    /// If loading banks made in FMOD Studio, this must be called with speakermode corresponding to the project output format
    /// if there is a possibility of the output audio device not matching the project format.
    /// Any differences between the project format and speakermode will cause the mix to sound wrong.
    ///
    /// By default speakermode will assume the setup the OS / output prefers.
    ///
    /// Altering the samplerate from the OS / output preferred rate may incur extra latency.
    /// Altering the speakermode from the OS / output preferred mode may cause an upmix/downmix which can alter the sound.
    ///
    /// On lower power platforms such as mobile samplerate will default to 24 kHz to reduce CPU cost.
    pub fn software_channels(&mut self, software_channels: c_int) -> Result<&mut Self> {
        unsafe {
            FMOD_System_SetSoftwareChannels(self.system, software_channels).to_result()?;
        };
        Ok(self)
    }

    /// Sets the buffer size for the FMOD software mixing engine.
    ///
    /// This function is used if you need to control mixer latency or granularity.
    /// Smaller buffersizes lead to smaller latency, but can lead to stuttering/skipping/unstable sound on slower machines or soundcards with bad drivers.
    ///
    /// To get the `buffer_size` in milliseconds, divide it by the output rate and multiply the result by 1000.
    /// For a `buffer_size` of 1024 and an output rate of 48khz (see [`Self::software_format`]), milliseconds = 1024 / 48000 * 1000 = 21.33ms.
    /// This means the mixer updates every 21.33ms.
    ///
    /// To get the total buffer size multiply the `buffer_size` by the numbuffers value.
    /// By default this would be 4 * 1024 = 4096 samples, or 4 * 21.33ms = 85.33ms.
    /// This would generally be the total latency of the software mixer,
    /// but in reality due to one of the buffers being written to constantly,
    /// and the cursor position of the buffer that is audible,
    /// the latency is typically more like the (number of buffers - 1.5) multiplied by the buffer length.
    ///
    /// To convert from milliseconds back to 'samples',
    /// simply multiply the value in milliseconds by the sample rate of the output (ie 48000 if that is what it is set to),
    /// then divide by 1000.
    ///
    /// The FMOD software mixer mixes to a ringbuffer.
    /// The size of this ringbuffer is determined here.
    /// It mixes a block of sound data every '`buffer_size`' number of samples,
    /// and there are 'numbuffers' number of these blocks that make up the entire ringbuffer.
    /// Adjusting these values can lead to extremely low latency performance (smaller values),
    /// or greater stability in sound output (larger values).
    ///
    /// ### Warning! The '`buffer_size`' is generally best left alone.
    ///
    /// Making the granularity smaller will just increase CPU usage (cache misses and DSP graph overhead).
    /// Making it larger affects how often you hear commands update such as volume/pitch/pan changes.
    /// Anything above 20ms will be noticeable and sound parameter changes will be obvious instead of smooth.
    ///
    /// FMOD chooses the most optimal size by default for best stability,
    /// depending on the output type. It is not recommended changing this value unless you really need to.
    /// You may get worse performance than the default settings chosen by FMOD.
    /// If you do set the size manually, the `buffer_size` argument must be a multiple of four,
    /// typically 256, 480, 512, 1024 or 2048 depedning on your latency requirements.
    pub fn dsp_buffer_size(
        &mut self,
        buffer_size: c_uint,
        buffer_count: c_int,
    ) -> Result<&mut Self> {
        unsafe {
            FMOD_System_SetDSPBufferSize(self.system, buffer_size, buffer_count).to_result()?;
        };
        Ok(self)
    }

    /// Sets the type of output interface used to run the mixer.
    ///
    /// This function is typically used to select between different OS specific audio APIs which may have different features.
    ///
    /// It is only necessary to call this function if you want to specifically switch away from the default output mode for the operating system.
    /// The most optimal mode is selected by default for the operating system.
    pub fn output(&mut self, kind: OutputType) -> Result<&mut Self> {
        unsafe {
            FMOD_System_SetOutput(self.system, kind.into()).to_result()?;
        };
        Ok(self)
    }

    /// Selects an output type given a plug-in handle.
    pub fn output_by_plugin(&mut self, handle: c_uint) -> Result<&mut Self> {
        unsafe {
            FMOD_System_SetOutputByPlugin(self.system, handle).to_result()?;
        };
        Ok(self)
    }

    /// Initialize the system object and prepare FMOD for playback.
    pub fn build(self, max_channels: c_int, flags: InitFlags) -> Result<System> {
        unsafe { self.build_with_extra_driver_data(max_channels, flags, std::ptr::null_mut()) }
    }

    /// # Safety
    ///
    /// See the FMOD docs explaining driver data for more safety information.
    pub unsafe fn build_with_extra_driver_data(
        self,
        max_channels: c_int,
        mut flags: InitFlags,
        driver_data: *mut c_void,
    ) -> Result<System> {
        if self.thread_unsafe {
            flags.insert(InitFlags::THREAD_UNSAFE);
        } else {
            #[cfg(not(feature = "thread-unsafe"))]
            flags.remove(InitFlags::THREAD_UNSAFE);
        }
        unsafe {
            FMOD_System_Init(self.system, max_channels, flags.bits(), driver_data).to_result()?;
            Ok(System::from_ffi(self.system))
        }
    }
}
