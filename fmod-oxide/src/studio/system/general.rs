// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::{Utf8CStr, Utf8CString};
use std::mem::MaybeUninit;

use crate::Guid;
use crate::studio::{System, get_string_out_size};
use crate::{FmodResultExt, Result};

impl System {
    /// Retrieves the Core System.
    pub fn get_core_system(&self) -> Result<crate::core::System> {
        let mut system = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetCoreSystem(self.inner.as_ptr(), &raw mut system).to_result()?;
            Ok(crate::core::System::from_ffi(system))
        }
    }

    /// Retrieves the ID for a bank, event, snapshot, bus or VCA.
    ///
    /// The strings bank must be loaded prior to calling this function, otherwise [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned.
    ///
    /// The path can be copied to the system clipboard from FMOD Studio using the "Copy Path" context menu command.
    pub fn lookup_id(&self, path: &Utf8CStr) -> Result<Guid> {
        let mut guid = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_System_LookupID(self.inner.as_ptr(), path.as_ptr(), guid.as_mut_ptr())
                .to_result()?;

            let guid = guid.assume_init().into();
            Ok(guid)
        }
    }

    /// Retrieves the path for a bank, event, snapshot, bus or VCA.
    ///
    /// The strings bank must be loaded prior to calling this function, otherwise [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned.
    pub fn lookup_path(&self, id: Guid) -> Result<Utf8CString> {
        get_string_out_size(|path, size, ret| unsafe {
            FMOD_Studio_System_LookupPath(self.inner.as_ptr(), &id.into(), path, size, ret)
        })
    }

    /// Checks that the [`System`] reference is valid and has been initialized.
    pub fn is_valid(&self) -> bool {
        unsafe { FMOD_Studio_System_IsValid(self.inner.as_ptr()).into() }
    }
}
