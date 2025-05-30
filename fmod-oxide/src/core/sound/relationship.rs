// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::c_int;

use crate::{FmodResultExt, Result};
use fmod_sys::*;

use crate::{Sound, SoundGroup};

impl Sound {
    /// Moves the sound from its existing [`SoundGroup`] to the specified sound group.
    ///
    /// By default, a sound is located in the 'master sound group'.
    /// This can be retrieved with `System::getMasterSoundGroup`.
    pub fn set_sound_group(&self, group: SoundGroup) -> Result<()> {
        unsafe { FMOD_Sound_SetSoundGroup(self.inner.as_ptr(), group.into()).to_result() }
    }

    /// Retrieves the sound's current sound group.
    pub fn sound_group(&self) -> Result<SoundGroup> {
        let mut group = std::ptr::null_mut();
        unsafe {
            FMOD_Sound_GetSoundGroup(self.inner.as_ptr(), &raw mut group).to_result()?;
            Ok(SoundGroup::from_ffi(group))
        }
    }

    /// Retrieves the number of subsounds stored within a sound.
    ///
    /// A format that has subsounds is a container format, such as FSB, DLS, MOD, S3M, XM, IT.
    pub fn get_sub_sound_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Sound_GetNumSubSounds(self.inner.as_ptr(), &raw mut count).to_result()?;
        }
        Ok(count)
    }

    /// Retrieves a handle to a Sound object that is contained within the parent sound.
    ///
    /// If the sound is a stream and `FMOD_NONBLOCKING` was not used,
    /// then this call will perform a blocking seek/flush to the specified subsound.
    ///
    /// If `FMOD_NONBLOCKING` was used to open this sound and the sound is a stream,
    /// FMOD will do a non blocking seek/flush and set the state of the subsound to `FMOD_OPENSTATE_SEEKING`.
    ///
    /// The sound won't be ready to be used when `FMOD_NONBLOCKING` is used,
    /// until the state of the sound becomes `FMOD_OPENSTATE_READY` or `FMOD_OPENSTATE_ERROR`.
    pub fn get_sub_sound(&self, index: c_int) -> Result<Sound> {
        let mut sound = std::ptr::null_mut();
        unsafe {
            FMOD_Sound_GetSubSound(self.inner.as_ptr(), index, &raw mut sound).to_result()?;
            Ok(Sound::from_ffi(sound))
        }
    }

    /// Retrieves the parent Sound object that contains this subsound.
    pub fn get_sub_sound_parent(&self) -> Result<Option<Sound>> {
        let mut sound = std::ptr::null_mut();
        unsafe {
            FMOD_Sound_GetSubSoundParent(self.inner.as_ptr(), &raw mut sound).to_result()?;
        }
        if sound.is_null() {
            Ok(None)
        } else {
            Ok(Some(unsafe { Sound::from_ffi(sound) }))
        }
    }
}
