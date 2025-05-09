// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

use crate::studio::{EventDescription, EventInstance};

#[cfg(doc)]
use crate::studio::PlaybackState;

#[cfg(fmod_2_3)]
use crate::studio::System;
use crate::{FmodResultExt, Result};

impl EventInstance {
    /// Retrieves the event description.
    pub fn get_description(&self) -> Result<EventDescription> {
        let mut description = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_EventInstance_GetDescription(self.inner.as_ptr(), &raw mut description)
                .to_result()?;
            Ok(EventDescription::from_ffi(description))
        }
    }

    /// Marks the event instance for release.
    ///
    /// This function marks the event instance to be released.
    /// Event instances marked for release are destroyed by the asynchronous update when they are in the stopped state ([`PlaybackState::Stopped`]).
    ///
    /// Generally it is a best practice to release event instances immediately after calling [`EventInstance::start`],
    /// unless you want to play the event instance multiple times or explicitly stop it and start it again later.
    /// It is possible to interact with the instance after falling [`EventInstance::release`], however if the sound has stopped [`FMOD_RESULT::FMOD_ERR_INVALID_HANDLE`] will be returned.
    pub fn release(&self) -> Result<()> {
        // we don't actually release userdata here because there is a callback, and the user might interact with the instance while it's being released
        unsafe { FMOD_Studio_EventInstance_Release(self.inner.as_ptr()).to_result() }
    }

    /// Checks that the [`EventInstance`] reference is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { FMOD_Studio_EventInstance_IsValid(self.inner.as_ptr()).into() }
    }

    /// Retrieves the FMOD Studio [`System`].
    #[cfg(fmod_2_3)]
    pub fn get_system(&self) -> Result<System> {
        let mut system = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_EventInstance_GetSystem(self.inner.as_ptr(), &raw mut system)
                .to_result()?;
            Ok(System::from_ffi(system))
        }
    }
}
