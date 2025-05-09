// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{Error, FmodResultExt, Result};
use std::ffi::{c_int, c_uint, c_void};

use fmod_sys::*;
use lanyard::Utf8CStr;

#[cfg(feature = "studio")]
use crate::studio;
use crate::{
    Channel, ChannelControl, ChannelGroup, Dsp, DspConnection, Geometry, OutputType, Reverb3D,
    Sound, SoundGroup, System, panic_wrapper,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorCallbackInfo<'a> {
    pub error: Error,
    pub instance: Instance,
    pub function_name: &'a Utf8CStr,
    pub function_params: &'a Utf8CStr,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instance {
    None,
    System(System),
    Channel(Channel),
    ChannelGroup(ChannelGroup),
    ChannelControl(ChannelControl),
    Sound(Sound),
    SoundGroup(SoundGroup),
    Dsp(Dsp),
    DspConnection(DspConnection),
    Geometry(Geometry),
    Reverb3D(Reverb3D),
    StudioParameterInstance,

    #[cfg(feature = "studio")]
    StudioSystem(studio::System),
    #[cfg(feature = "studio")]
    StudioEventDescription(studio::EventDescription),
    #[cfg(feature = "studio")]
    StudioEventInstance(studio::EventInstance),
    #[cfg(feature = "studio")]
    StudioBus(studio::Bus),
    #[cfg(feature = "studio")]
    StudioVCA(studio::Vca),
    #[cfg(feature = "studio")]
    StudioBank(studio::Bank),
    #[cfg(feature = "studio")]
    StudioCommandReplay(studio::CommandReplay),

    #[cfg(not(feature = "studio"))]
    StudioSystem(*mut c_void),
    #[cfg(not(feature = "studio"))]
    StudioEventDescription(*mut c_void),
    #[cfg(not(feature = "studio"))]
    StudioEventInstance(*mut c_void),
    #[cfg(not(feature = "studio"))]
    StudioBus(*mut c_void),
    #[cfg(not(feature = "studio"))]
    StudioVCA(*mut c_void),
    #[cfg(not(feature = "studio"))]
    StudioBank(*mut c_void),
    #[cfg(not(feature = "studio"))]
    StudioCommandReplay(*mut c_void),
}

impl Instance {
    fn from_raw(kind: c_uint, pointer: *mut c_void) -> Self {
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

#[allow(unused_variables)]
pub trait SystemCallback {
    fn device_list_changed(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn device_lost(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn memory_allocation_failed(
        system: System,
        file: &Utf8CStr,
        size: c_int,
        userdata: *mut c_void,
    ) -> Result<()> {
        Ok(())
    }

    fn thread_created(
        system: System,
        handle: *mut c_void,
        thread_name: &Utf8CStr,
        userdata: *mut c_void,
    ) -> Result<()> {
        Ok(())
    }

    fn bad_dsp_connection(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn premix(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn postmix(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

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

    fn thread_destroyed(
        system: System,
        handle: *mut c_void,
        thread_name: &Utf8CStr,
        userdata: *mut c_void,
    ) -> Result<()> {
        Ok(())
    }

    fn pre_update(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn post_update(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn record_list_changed(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn buffered_no_mix(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

    fn device_reinitialize(
        system: System,
        output_type: OutputType,
        driver_index: c_int,
        userdata: *mut c_void,
    ) -> Result<()> {
        Ok(())
    }

    fn output_underrun(system: System, userdata: *mut c_void) -> Result<()> {
        Ok(())
    }

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
