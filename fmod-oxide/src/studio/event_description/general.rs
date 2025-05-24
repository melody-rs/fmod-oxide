// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{ffi::c_int, mem::MaybeUninit};

use fmod_sys::*;
use lanyard::Utf8CString;

use crate::Guid;
use crate::studio::{EventDescription, get_string_out_size};
use crate::{FmodResultExt, Result};

impl EventDescription {
    /// Retrieves the GUID.
    pub fn get_id(&self) -> Result<Guid> {
        let mut guid = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_EventDescription_GetID(self.as_ptr(), guid.as_mut_ptr())
                .to_result()?;

            let guid = guid.assume_init().into();

            Ok(guid)
        }
    }

    /// Retrieves the length of the timeline.
    ///
    /// A timeline's length is the largest of any logic markers, transition leadouts and the end of any trigger boxes on the timeline.
    pub fn get_length(&self) -> Result<c_int> {
        let mut length = 0;
        unsafe {
            FMOD_Studio_EventDescription_GetLength(self.as_ptr(), &raw mut length)
                .to_result()?;
        }
        Ok(length)
    }

    /// Retrieves the path.
    ///
    /// The strings bank must be loaded prior to calling this function, otherwise [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned.
    pub fn get_path(&self) -> Result<Utf8CString> {
        get_string_out_size(|path, size, ret| unsafe {
            FMOD_Studio_EventDescription_GetPath(self.as_ptr(), path, size, ret)
        })
    }

    /// Checks that the [`EventDescription`] reference is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { FMOD_Studio_EventDescription_IsValid(self.as_ptr()).into() }
    }
}
