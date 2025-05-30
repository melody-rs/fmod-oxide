// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::{Utf8CStr, Utf8CString};
use std::{
    ffi::{c_char, c_float, c_int},
    mem::MaybeUninit,
};

use crate::{FmodResultExt, Result};
use crate::{
    get_string,
    studio::{CommandInfo, CommandReplay, System},
};

impl CommandReplay {
    /// Sets a path substition that will be used when loading banks with this replay.
    ///
    /// [`System::load_bank_file`] commands in the replay are redirected to load banks from the specified directory, instead of using the directory recorded in the captured commands.
    pub fn set_bank_path(&self, path: &Utf8CStr) -> Result<()> {
        unsafe {
            FMOD_Studio_CommandReplay_SetBankPath(self.inner.as_ptr(), path.as_ptr()).to_result()
        }
    }

    /// Retrieves the command index corresponding to the given playback time.
    ///
    /// This function will return an index for the first command at or after `time`.
    /// If `time` is greater than the total playback time then [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned.
    pub fn command_at_time(&self, time: c_float) -> Result<c_int> {
        let mut index = 0;
        unsafe {
            FMOD_Studio_CommandReplay_GetCommandAtTime(self.inner.as_ptr(), time, &raw mut index)
                .to_result()?;
        }
        Ok(index)
    }

    /// Retrieves the number of commands in the replay.
    pub fn get_command_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_CommandReplay_GetCommandCount(self.inner.as_ptr(), &raw mut count)
                .to_result()?;
        }
        Ok(count)
    }

    /// Retrieves command information.
    pub fn get_command_info(&self, index: c_int) -> Result<CommandInfo> {
        let mut info = MaybeUninit::zeroed();

        unsafe {
            FMOD_Studio_CommandReplay_GetCommandInfo(self.inner.as_ptr(), index, info.as_mut_ptr())
                .to_result()?;

            let info = CommandInfo::from_ffi(info.assume_init());
            Ok(info)
        }
    }

    /// Retrieves the string representation of a command.
    pub fn get_command_string(&self, index: c_int) -> Result<Utf8CString> {
        let string = get_string(|buffer| unsafe {
            FMOD_Studio_CommandReplay_GetCommandString(
                self.inner.as_ptr(),
                index,
                buffer.as_mut_ptr().cast::<c_char>(),
                buffer.len() as c_int,
            )
        })?;

        Ok(string)
    }

    /// Retrieves the total playback time.
    pub fn get_length(&self) -> Result<c_float> {
        let mut length = 0.0;
        unsafe {
            FMOD_Studio_CommandReplay_GetLength(self.inner.as_ptr(), &raw mut length)
                .to_result()?;
        }
        Ok(length)
    }

    /// Retrieves the Studio System object associated with this replay object.
    pub fn get_system(&self) -> Result<System> {
        let mut system = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_CommandReplay_GetSystem(self.inner.as_ptr(), &raw mut system)
                .to_result()?;
            Ok(System::from_ffi(system))
        }
    }

    /// Checks that the [`CommandReplay`] reference is valid.
    pub fn is_valid(&self) -> bool {
        unsafe { FMOD_Studio_CommandReplay_IsValid(self.inner.as_ptr()).into() }
    }
}
