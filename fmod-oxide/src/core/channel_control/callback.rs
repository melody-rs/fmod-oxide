// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::{
    ffi::{c_float, c_int},
    ops::Deref,
    os::raw::c_void,
};

use crate::{Channel, ChannelControl, ChannelGroup, panic_wrapper};
use crate::{FmodResultExt, Result};

/// Enum used to distinguish between [`Channel`] and [`ChannelGroup`] in the [`ChannelControl`] callback.
#[derive(Debug, Clone, Copy)]
pub enum ChannelControlType {
    /// [`ChannelControl`] is a [`Channel`].
    Channel(Channel),
    /// [`ChannelControl`] is a [`ChannelGroup`].
    ChannelGroup(ChannelGroup),
}

/// Trait for this particular FMOD callback.
///
/// No `self` parameter is passed to the callback!
#[allow(unused_variables)]
pub trait ChannelControlCallback {
    /// Called when a sound ends. Supported by [`Channel`] only.
    fn end(channel_control: ChannelControlType) -> Result<()> {
        Ok(())
    }

    /// Called when a [`Channel`] is made virtual or real. Supported by [`Channel`] objects only.
    fn virtual_voice(channel_control: ChannelControlType, is_virtual: bool) -> Result<()> {
        Ok(())
    }

    /// Called when a syncpoint is encountered.
    /// Can be from wav file markers or user added.
    /// Supported by [`Channel`] only.
    fn sync_point(channel_control: ChannelControlType, sync_point: c_int) -> Result<()> {
        Ok(())
    }

    /// Called when geometry occlusion values are calculated.
    /// Can be used to clamp or change the value.
    /// Supported by [`Channel`] and [`ChannelGroup`].
    // FIXME: is this &mut safe?
    fn occlusion(
        channel_control: ChannelControlType,
        direct: &mut c_float,
        reverb: &mut c_float,
    ) -> Result<()> {
        Ok(())
    }
}

impl Deref for ChannelControlType {
    type Target = ChannelControl;

    fn deref(&self) -> &Self::Target {
        match self {
            ChannelControlType::Channel(channel) => channel,
            ChannelControlType::ChannelGroup(channel_group) => channel_group,
        }
    }
}

unsafe extern "C" fn callback_impl<C: ChannelControlCallback>(
    channel_control: *mut FMOD_CHANNELCONTROL,
    control_type: FMOD_CHANNELCONTROL_TYPE,
    callback_type: FMOD_CHANNELCONTROL_CALLBACK_TYPE,
    commanddata1: *mut c_void,
    commanddata2: *mut c_void,
) -> FMOD_RESULT {
    panic_wrapper(|| {
        let channel_control = match control_type {
            FMOD_CHANNELCONTROL_CHANNEL => {
                let channel = unsafe { Channel::from_ffi(channel_control.cast()) };
                ChannelControlType::Channel(channel)
            }
            FMOD_CHANNELCONTROL_CHANNELGROUP => {
                let channel_group = unsafe { ChannelGroup::from_ffi(channel_control.cast()) };
                ChannelControlType::ChannelGroup(channel_group)
            }
            _ => return FMOD_RESULT::FMOD_ERR_INVALID_PARAM, // this should never happen
        };

        let result = match callback_type {
            FMOD_CHANNELCONTROL_CALLBACK_END => C::end(channel_control),
            FMOD_CHANNELCONTROL_CALLBACK_VIRTUALVOICE => {
                let is_virtual = unsafe { *commanddata1.cast::<i32>() } != 0;
                C::virtual_voice(channel_control, is_virtual)
            }
            FMOD_CHANNELCONTROL_CALLBACK_SYNCPOINT => {
                let sync_point = unsafe { *commanddata1.cast::<c_int>() };
                C::sync_point(channel_control, sync_point)
            }
            FMOD_CHANNELCONTROL_CALLBACK_OCCLUSION => {
                let direct = unsafe { &mut *commanddata1.cast::<c_float>() };
                let reverb = unsafe { &mut *commanddata2.cast::<c_float>() };
                C::occlusion(channel_control, &mut *direct, &mut *reverb)
            }
            _ => {
                eprintln!("warning: unknown callback type {callback_type}");
                return FMOD_RESULT::FMOD_OK;
            }
        };
        FMOD_RESULT::from_result(result)
    })
}

impl ChannelControl {
    /// Sets the callback for [`ChannelControl`] level notifications.
    pub fn set_callback<C: ChannelControlCallback>(&self) -> Result<()> {
        unsafe {
            FMOD_ChannelControl_SetCallback(self.inner.as_ptr(), Some(callback_impl::<C>))
                .to_result()
        }
    }
}
