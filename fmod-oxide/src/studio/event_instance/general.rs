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
            FMOD_Studio_EventInstance_GetDescription(self.as_ptr(), &raw mut description)
                .to_result()?;
            Ok(EventDescription::from_ffi(description))
        }
    }

    /// Checks that the [`EventInstance`] reference is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { FMOD_Studio_EventInstance_IsValid(self.as_ptr()).into() }
    }

    /// Retrieves the FMOD Studio [`System`].
    #[cfg(fmod_2_3)]
    pub fn get_system(&self) -> Result<&System> {
        let mut system = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_EventInstance_GetSystem(self.as_ptr(), &raw mut system).to_result()?;
            Ok(System::from_ffi(system))
        }
    }
}
