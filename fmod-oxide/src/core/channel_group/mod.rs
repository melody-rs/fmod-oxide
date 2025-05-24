// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use std::{ops::Deref, ptr::NonNull};

use crate::ChannelControl;

mod channel_management;
mod general;
mod group_management;

#[cfg(doc)]
use crate::{Channel, System};

/// A submix in the mixing hierarchy akin to a bus that can contain both [`Channel`] and [`ChannelGroup`] objects.
///
/// Create with [`System::create_channel_group`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct ChannelGroup {
    pub(crate) inner: NonNull<FMOD_CHANNELGROUP>,
}

#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for ChannelGroup {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for ChannelGroup {}

impl ChannelGroup {
    /// # Safety
    ///
    /// `value` must be a valid pointer either aquired from [`Self::as_ptr`] or FMOD.
    ///
    /// # Panics
    ///
    /// Panics if `value` is null.
    pub unsafe fn from_ffi(value: *mut FMOD_CHANNELGROUP) -> Self {
        let inner = NonNull::new(value).unwrap();
        ChannelGroup { inner }
    }

    /// Converts `self` into its raw representation.
    pub fn as_ptr(&self) -> *mut FMOD_CHANNELGROUP {
        std::ptr::from_ref(self).cast_mut().cast()
    }
}

impl From<ChannelGroup> for *mut FMOD_CHANNELGROUP {
    fn from(value: ChannelGroup) -> Self {
        value.inner.as_ptr()
    }
}

impl Deref for ChannelGroup {
    type Target = ChannelControl;

    fn deref(&self) -> &Self::Target {
        #[cfg(debug_assertions)]
        unsafe {
            // perform a debug check to ensure that the the cast results in the same pointer
            let control = FMOD_ChannelGroup_CastToControl(self.as_ptr());
            assert_eq!(
                control as usize,
                self.as_ptr() as usize,
                "ChannelControl cast was not equivalent! THIS IS A MAJOR BUG. PLEASE REPORT THIS!"
            );
        }
        // channelcontrol has the same layout as channel, and if the assumption in channel_control.rs is correct,
        // this is cast is safe.
        unsafe { &*std::ptr::from_ref(self).cast::<ChannelControl>() }
    }
}
