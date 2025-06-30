// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![allow(missing_docs, deprecated)]

use crate::{Error, FmodResultExt, Result};
use std::ffi::{c_int, c_void};

use fmod_sys::*;
use lanyard::Utf8CStr;

#[cfg(feature = "studio")]
use crate::studio;
use crate::{
    Channel, ChannelControl, ChannelGroup, Dsp, DspConnection, Geometry, OutputType, Reverb3D,
    Sound, SoundGroup, System, panic_wrapper,
};

/// Information describing an error that has occurred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorCallbackInfo<'a> {
    /// Error code result.
    pub error: Error,
    /// Type of instance the error occurred on.
    pub instance: Instance,
    /// Function that the error occurred on.
    pub function_name: &'a Utf8CStr,
    /// Function parameters that the error ocurred on.
    pub function_params: &'a Utf8CStr,
}

/// Identifier used to represent the different types of instance in the error callback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instance {
    /// Type representing no known instance type.
    None,
    /// Type representing [`System`].
    System(System),
    /// Type representing [`Channel`].
    Channel(Channel),
    ChannelGroup(ChannelGroup),
    /// Type representing [`ChannelControl`].
    ChannelControl(ChannelControl),
    Sound(Sound),
    /// Type representing [`Sound`].
    SoundGroup(SoundGroup),
    /// Type representing [`Dsp`].
    Dsp(Dsp),
    /// Type representing [`DspConnection`].
    DspConnection(DspConnection),
    /// Type representing [`Geometry`].
    Geometry(Geometry),
    /// Type representing [`Reverb3D`].
    Reverb3D(Reverb3D),
    /// Deprecated.
    #[deprecated]
    StudioParameterInstance,

    /// Type representing [`studio::System`].
    #[cfg(feature = "studio")]
    StudioSystem(studio::System),
    /// Type representing [`studio::EventDescription`].
    #[cfg(feature = "studio")]
    StudioEventDescription(studio::EventDescription),
    #[cfg(feature = "studio")]
    /// Type representing [`studio::EventInstance`].
    StudioEventInstance(studio::EventInstance),
    #[cfg(feature = "studio")]
    /// Type representing [`studio::Bus`].
    StudioBus(studio::Bus),
    #[cfg(feature = "studio")]
    /// Type representing [`studio::Vca`].
    StudioVCA(studio::Vca),
    #[cfg(feature = "studio")]
    /// Type representing [`studio::Bank`].
    StudioBank(studio::Bank),
    #[cfg(feature = "studio")]
    /// Type representing [`studio::CommandReplay`].
    StudioCommandReplay(studio::CommandReplay),

    /// Represents a raw FMOD Studio type.
    /// Because the Studio feature is disabled, you shouldn't be able to get this variant.
    #[cfg(not(feature = "studio"))]
    /// Represents a raw FMOD Studio type.
    /// Because the Studio feature is disabled, you shouldn't be able to get this variant.
    StudioSystem(*mut c_void),
    #[cfg(not(feature = "studio"))]
    /// Represents a raw FMOD Studio type.
    /// Because the Studio feature is disabled, you shouldn't be able to get this variant.
    StudioEventDescription(*mut c_void),
    #[cfg(not(feature = "studio"))]
    /// Represents a raw FMOD Studio type.
    /// Because the Studio feature is disabled, you shouldn't be able to get this variant.
    StudioEventInstance(*mut c_void),
    #[cfg(not(feature = "studio"))]
    /// Represents a raw FMOD Studio type.
    /// Because the Studio feature is disabled, you shouldn't be able to get this variant.
    StudioBus(*mut c_void),
    #[cfg(not(feature = "studio"))]
    /// Represents a raw FMOD Studio type.
    /// Because the Studio feature is disabled, you shouldn't be able to get this variant.
    StudioVCA(*mut c_void),
    #[cfg(not(feature = "studio"))]
    /// Represents a raw FMOD Studio type.
    /// Because the Studio feature is disabled, you shouldn't be able to get this variant.
    StudioBank(*mut c_void),
    /// Represents a raw FMOD Studio type.
    /// Because the Studio feature is disabled, you shouldn't be able to get this variant.
    #[cfg(not(feature = "studio"))]
    StudioCommandReplay(*mut c_void),
}

impl Instance {
    fn from_raw(kind: FMOD_ERRORCALLBACK_INSTANCETYPE, pointer: *mut c_void) -> Self {
        match kind {
            FMOD_ERRORCALLBACK_INSTANCETYPE_NONE => Instance::None,
            FMOD_ERRORCALLBACK_INSTANCETYPE_SYSTEM => {
                Instance::System(unsafe { System::from_ffi(pointer.cast()) })
            }
            FMOD_ERRORCALLBACK_INSTANCETYPE_CHANNEL => {
                Instance::Channel(unsafe { Channel::from_ffi(pointer.cast()) })
            }
            FMOD_ERRORCALLBACK_INSTANCETYPE_CHANNELGROUP => {
                Instance::ChannelGroup(unsafe { ChannelGroup::from_ffi(pointer.cast()) })
            }
            FMOD_ERRORCALLBACK_INSTANCETYPE_CHANNELCONTROL => {
                Instance::ChannelControl(unsafe { ChannelControl::from_ffi(pointer.cast()) })
            }
            FMOD_ERRORCALLBACK_INSTANCETYPE_SOUND => {
                Instance::Sound(unsafe { Sound::from_ffi(pointer.cast()) })
            }
            FMOD_ERRORCALLBACK_INSTANCETYPE_SOUNDGROUP => {
                Instance::SoundGroup(unsafe { SoundGroup::from_ffi(pointer.cast()) })
            }
            FMOD_ERRORCALLBACK_INSTANCETYPE_DSP => {
                Instance::Dsp(unsafe { Dsp::from_ffi(pointer.cast()) })
            }
            FMOD_ERRORCALLBACK_INSTANCETYPE_DSPCONNECTION => {
                Instance::DspConnection(unsafe { DspConnection::from_ffi(pointer.cast()) })
            }
            FMOD_ERRORCALLBACK_INSTANCETYPE_GEOMETRY => {
                Instance::Geometry(unsafe { Geometry::from_ffi(pointer.cast()) })
            }
            FMOD_ERRORCALLBACK_INSTANCETYPE_REVERB3D => {
                Instance::Reverb3D(unsafe { Reverb3D::from_ffi(pointer.cast()) })
            }
            FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_PARAMETERINSTANCE => {
                Instance::StudioParameterInstance
            }
            #[cfg(not(feature = "studio"))]
            FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_SYSTEM => Instance::StudioSystem(pointer),
            #[cfg(not(feature = "studio"))]
            FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_EVENTDESCRIPTION => {
                Instance::StudioEventDescription(pointer)
            }
            #[cfg(not(feature = "studio"))]
            FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_EVENTINSTANCE => {
                Instance::StudioEventInstance(pointer)
            }
            #[cfg(not(feature = "studio"))]
            FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_BUS => Instance::StudioBus(pointer),
            #[cfg(not(feature = "studio"))]
            FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_VCA => Instance::StudioVCA(pointer),
            #[cfg(not(feature = "studio"))]
            FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_BANK => Instance::StudioBank(pointer),
            #[cfg(not(feature = "studio"))]
            FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_COMMANDREPLAY => {
                Instance::StudioCommandReplay(pointer)
            }
            #[cfg(feature = "studio")]
            FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_SYSTEM => {
                Instance::StudioSystem(unsafe { studio::System::from_ffi(pointer.cast()) })
            }
            #[cfg(feature = "studio")]
            FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_EVENTDESCRIPTION => {
                Instance::StudioEventDescription(unsafe {
                    studio::EventDescription::from_ffi(pointer.cast())
                })
            }
            #[cfg(feature = "studio")]
            FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_EVENTINSTANCE => {
                Instance::StudioEventInstance(unsafe {
                    studio::EventInstance::from_ffi(pointer.cast())
                })
            }
            #[cfg(feature = "studio")]
            FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_BUS => {
                Instance::StudioBus(unsafe { studio::Bus::from_ffi(pointer.cast()) })
            }
            #[cfg(feature = "studio")]
            FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_VCA => {
                Instance::StudioVCA(unsafe { studio::Vca::from_ffi(pointer.cast()) })
            }
            #[cfg(feature = "studio")]
            FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_BANK => {
                Instance::StudioBank(unsafe { studio::Bank::from_ffi(pointer.cast()) })
            }
            #[cfg(feature = "studio")]
            FMOD_ERRORCALLBACK_INSTANCETYPE_STUDIO_COMMANDREPLAY => {
                Instance::StudioCommandReplay(unsafe {
                    studio::CommandReplay::from_ffi(pointer.cast())
                })
            }
            _ => {
                eprintln!("warning: unknown instance type {kind}");
                Instance::None
            }
        }
    }
}

impl ErrorCallbackInfo<'_> {
    /// # Safety
    ///
    /// The function name and function params fields of [`FMOD_ERRORCALLBACK_INFO`] must be a null-terminated and must be valid for reads of bytes up to and including the nul terminator.
    ///
    /// See [`Utf8CStr::from_ptr_unchecked`] for more information.
    pub unsafe fn from_ffi(value: FMOD_ERRORCALLBACK_INFO) -> Self {
        Self {
            error: value.result.into(),
            instance: Instance::from_raw(value.instancetype, value.instance),
            function_name: unsafe { Utf8CStr::from_ptr_unchecked(value.functionname) },
            function_params: unsafe { Utf8CStr::from_ptr_unchecked(value.functionparams) },
        }
    }
}

bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
  pub struct SystemCallbackMask: FMOD_SYSTEM_CALLBACK_TYPE {
      const DEVICELISTCHANGED     = FMOD_SYSTEM_CALLBACK_DEVICELISTCHANGED;
      const DEVICELOST            = FMOD_SYSTEM_CALLBACK_DEVICELOST;
      const MEMORYALLOCATIONFAILED= FMOD_SYSTEM_CALLBACK_MEMORYALLOCATIONFAILED;
      const THREADCREATED         = FMOD_SYSTEM_CALLBACK_THREADCREATED;
      const BADDSPCONNECTION      = FMOD_SYSTEM_CALLBACK_BADDSPCONNECTION;
      const PREMIX                = FMOD_SYSTEM_CALLBACK_PREMIX;
      const POSTMIX               = FMOD_SYSTEM_CALLBACK_POSTMIX;
      const ERROR                 = FMOD_SYSTEM_CALLBACK_ERROR;
      #[cfg(fmod_eq_2_2)]
      const MIDMIX                = FMOD_SYSTEM_CALLBACK_MIDMIX;
      const THREADDESTROYED       = FMOD_SYSTEM_CALLBACK_THREADDESTROYED;
      const PREUPDATE             = FMOD_SYSTEM_CALLBACK_PREUPDATE;
      const POSTUPDATE            = FMOD_SYSTEM_CALLBACK_POSTUPDATE;
      const RECORDLISTCHANGED     = FMOD_SYSTEM_CALLBACK_RECORDLISTCHANGED;
      const BUFFEREDNOMIX         = FMOD_SYSTEM_CALLBACK_BUFFEREDNOMIX;
      const DEVICEREINITIALIZE    = FMOD_SYSTEM_CALLBACK_DEVICEREINITIALIZE;
      const OUTPUTUNDERRUN        = FMOD_SYSTEM_CALLBACK_OUTPUTUNDERRUN;
      const RECORDPOSITIONCHANGED = FMOD_SYSTEM_CALLBACK_RECORDPOSITIONCHANGED ;
      const ALL                   = FMOD_SYSTEM_CALLBACK_ALL;
  }
}

impl From<SystemCallbackMask> for FMOD_SYSTEM_CALLBACK_TYPE {
    fn from(mask: SystemCallbackMask) -> Self {
        mask.bits()
    }
}

impl From<FMOD_SYSTEM_CALLBACK_TYPE> for SystemCallbackMask {
    fn from(mask: FMOD_SYSTEM_CALLBACK_TYPE) -> Self {
        Self::from_bits_truncate(mask)
    }
}

/// Trait for this particular FMOD callback.
///
/// No `self` parameter is passed to the callback!
#[allow(unused_variables)]
pub trait SystemCallback {
    /// Called from [`System::update`] when the enumerated list of devices has changed.
    ///
    /// Called from the main (calling) thread when set from the Core API or Studio API in synchronous mode,
    /// and from the Studio Update Thread when in default / async mode.
    fn device_list_changed(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    /// Deprecated.
    #[deprecated]
    fn device_lost(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    /// Called directly when a memory allocation fails.
    fn memory_allocation_failed(
        system: System,
        file: &Utf8CStr,
        size: c_int,
        userdata: *mut c_void,
    ) -> Result<()> {
        Ok(())
    }

    /// Called from the game thread when a thread is created.
    fn thread_created(
        system: System,
        handle: *mut c_void,
        thread_name: &Utf8CStr,
        userdata: *mut c_void,
    ) -> Result<()> {
        Ok(())
    }

    /// Deprecated.
    #[deprecated]
    fn bad_dsp_connection(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    /// Called from the mixer thread before it starts the next block.
    fn premix(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    /// Called from the mixer thread after it finishes a block.
    fn postmix(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    /// Called directly when an API function returns an error, including delayed async functions.
    fn error(
        system: System,
        error_info: ErrorCallbackInfo<'_>,
        userdata: *mut c_void,
    ) -> Result<()> {
        Ok(())
    }

    #[cfg(fmod_eq_2_2)]
    fn mid_mix(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    /// Called from the game thread when a thread is destroyed.
    fn thread_destroyed(
        system: System,
        handle: *mut c_void,
        thread_name: &Utf8CStr,
        userdata: *mut c_void,
    ) -> Result<()> {
        Ok(())
    }

    /// Called at start of [`System::update`] from the main (calling)
    /// thread when set from the Core API or Studio API in synchronous mode,
    /// and from the Studio Update Thread when in default / async mode.
    fn pre_update(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    /// Called at end of [`System::update`] from the main (calling)
    /// thread when set from the Core API or Studio API in synchronous mode,
    /// and from the Studio Update Thread when in default / async mode.
    fn post_update(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    /// Called from [`System::update`] when the enumerated list of recording devices has changed.
    /// Called from the main (calling) thread when set from the Core API or Studio API in synchronous mode,
    /// and from the Studio Update Thread when in default / async mode.
    fn record_list_changed(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    /// Called from the feeder thread after audio was consumed from the ring buffer,
    /// but not enough to allow another mix to run.
    fn buffered_no_mix(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    /// Called from [`System::update`] when an output device is re-initialized.
    /// Called from the main (calling) thread when set from the Core API or Studio API in synchronous mode,
    /// and from the Studio Update Thread when in default / async mode.
    fn device_reinitialize(
        system: System,
        output_type: OutputType,
        driver_index: c_int,
        userdata: *mut c_void,
    ) -> Result<()> {
        Ok(())
    }

    /// Called from the mixer thread when the device output attempts to read more samples than are available in the output buffer.
    fn output_underrun(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    /// Called from the mixer thread when the System record position changed.
    fn record_position_changed(
        system: System,
        sound: Sound,
        record_position: c_int,
        userdata: *mut c_void,
    ) -> Result<()> {
        Ok(())
    }
}

unsafe extern "C" fn callback_impl<C: SystemCallback>(
    system: *mut FMOD_SYSTEM,
    callback_type: FMOD_SYSTEM_CALLBACK_TYPE,
    command_data_1: *mut c_void,
    command_data_2: *mut c_void,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    #[allow(deprecated)]
    panic_wrapper(|| {
        let system = unsafe { System::from_ffi(system) };
        let result = match callback_type {
            FMOD_SYSTEM_CALLBACK_DEVICELISTCHANGED => C::device_list_changed(system, userdata),
            FMOD_SYSTEM_CALLBACK_DEVICELOST => C::device_lost(system, userdata),
            FMOD_SYSTEM_CALLBACK_MEMORYALLOCATIONFAILED => {
                let file = unsafe { Utf8CStr::from_ptr_unchecked(command_data_1.cast()) };
                C::memory_allocation_failed(system, file, command_data_2 as c_int, userdata)
            }
            FMOD_SYSTEM_CALLBACK_THREADCREATED => {
                let thread_name = unsafe { Utf8CStr::from_ptr_unchecked(command_data_2.cast()) };
                C::thread_created(system, command_data_1, thread_name, userdata)
            }
            FMOD_SYSTEM_CALLBACK_BADDSPCONNECTION => C::bad_dsp_connection(system, userdata),
            FMOD_SYSTEM_CALLBACK_PREMIX => C::premix(system, userdata),
            FMOD_SYSTEM_CALLBACK_POSTMIX => C::postmix(system, userdata),
            FMOD_SYSTEM_CALLBACK_ERROR => {
                let error_info = unsafe { ErrorCallbackInfo::from_ffi(*command_data_1.cast()) };
                C::error(system, error_info, userdata)
            }
            #[cfg(fmod_eq_2_2)]
            FMOD_SYSTEM_CALLBACK_MIDMIX => C::mid_mix(system, userdata),
            FMOD_SYSTEM_CALLBACK_THREADDESTROYED => {
                let thread_name = unsafe { Utf8CStr::from_ptr_unchecked(command_data_2.cast()) };
                C::thread_destroyed(system, command_data_1, thread_name, userdata)
            }
            FMOD_SYSTEM_CALLBACK_PREUPDATE => C::pre_update(system, userdata),
            FMOD_SYSTEM_CALLBACK_POSTUPDATE => C::post_update(system, userdata),
            FMOD_SYSTEM_CALLBACK_RECORDLISTCHANGED => C::record_list_changed(system, userdata),
            FMOD_SYSTEM_CALLBACK_BUFFEREDNOMIX => C::buffered_no_mix(system, userdata),
            FMOD_SYSTEM_CALLBACK_DEVICEREINITIALIZE => {
                let output_type = OutputType::try_from(command_data_1 as FMOD_OUTPUTTYPE)
                    .expect("invalid output type");
                C::device_reinitialize(system, output_type, command_data_2 as c_int, userdata)
            }
            FMOD_SYSTEM_CALLBACK_OUTPUTUNDERRUN => C::output_underrun(system, userdata),
            FMOD_SYSTEM_CALLBACK_RECORDPOSITIONCHANGED => {
                let sound = unsafe { Sound::from_ffi(command_data_1.cast()) };
                C::record_position_changed(system, sound, command_data_2 as c_int, userdata)
            }
            _ => {
                eprintln!("warning: unknown callback type {callback_type}");
                return FMOD_RESULT::FMOD_OK;
            }
        };
        FMOD_RESULT::from_result(result)
    })
}

impl System {
    pub fn set_callback<C: SystemCallback>(&self, mask: SystemCallbackMask) -> Result<()> {
        unsafe {
            FMOD_System_SetCallback(self.inner.as_ptr(), Some(callback_impl::<C>), mask.into())
                .to_result()
        }
    }
}
