// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::c_int;

use crate::{
    FileSystemAsync, FileSystemSync, async_filesystem_cancel, async_filesystem_read,
    filesystem_close, filesystem_open, filesystem_read, filesystem_seek,
};
use crate::{FmodResultExt, Result};
use fmod_sys::*;

use super::System;

#[cfg(doc)]
use crate::Sound;

impl System {
    /// Set callbacks to implement all file I/O instead of using the platform native method.
    ///
    /// Setting these callbacks have no effect on sounds loaded with [`FMOD_OPENMEMORY`] or [`FMOD_OPENUSER`].
    ///
    /// Setting blockalign to 0 will disable file buffering and cause every read to invoke the relevant callback (not recommended),
    /// current default is tuned for memory usage vs performance.
    /// Be mindful of the I/O capabilities of the platform before increasing this default.
    pub fn set_default_filesystem(&self, block_align: c_int) -> Result<()> {
        unsafe {
            FMOD_System_SetFileSystem(
                self.as_ptr(),
                None,
                None,
                None,
                None,
                None,
                None,
                block_align,
            )
            .to_result()
        }
    }

    /// Set callbacks to implement all file I/O instead of using the platform native method.
    ///
    /// Setting these callbacks have no effect on sounds loaded with [`FMOD_OPENMEMORY`] or [`FMOD_OPENUSER`].
    ///
    /// Setting blockalign to 0 will disable file buffering and cause every read to invoke the relevant callback (not recommended),
    /// current default is tuned for memory usage vs performance.
    /// Be mindful of the I/O capabilities of the platform before increasing this default.
    pub fn set_filesystem_sync<F: FileSystemSync>(&self, block_align: c_int) -> Result<()> {
        unsafe {
            FMOD_System_SetFileSystem(
                self.as_ptr(),
                Some(filesystem_open::<F>),
                Some(filesystem_close::<F>),
                Some(filesystem_read::<F>),
                Some(filesystem_seek::<F>),
                None,
                None,
                block_align,
            )
            .to_result()
        }
    }

    /// Set callbacks to implement all file I/O instead of using the platform native method.
    ///
    /// Setting these callbacks have no effect on sounds loaded with [`FMOD_OPENMEMORY`] or [`FMOD_OPENUSER`].
    ///
    /// Setting blockalign to 0 will disable file buffering and cause every read to invoke the relevant callback (not recommended),
    /// current default is tuned for memory usage vs performance.
    /// Be mindful of the I/O capabilities of the platform before increasing this default.
    /// - it is recommended to consult the 'asyncio' example for reference implementation.
    ///   There is also a tutorial on the subject, Asynchronous I/O.
    /// - [`FileSystemAsync::read`] allows the user to return immediately before the data is ready.
    ///   FMOD will either wait internally (see note below about thread safety), or continuously check in the streamer until data arrives.
    ///   It is the user's responsibility to provide data in time in the stream case, or the stream may stutter.
    ///   Data starvation can be detected with [`Sound::get_open_state`].
    /// - Important: If [`FileSystemAsync::read`] is processed in the main thread, then it will hang the application,
    ///   because FMOD will wait internally until data is ready, and the main thread will not be able to supply the data.
    ///   For this reason the user's file access should normally be from a separate thread.
    /// - A [`FileSystemAsync::cancel`] must either service or prevent an async read issued previously via [`FileSystemAsync::read`] before returning.
    pub fn set_filesystem_async<F: FileSystemAsync>(&self, block_align: c_int) -> Result<()> {
        unsafe {
            FMOD_System_SetFileSystem(
                self.as_ptr(),
                Some(filesystem_open::<F>),
                Some(filesystem_close::<F>),
                None,
                None,
                Some(async_filesystem_read::<F>),
                Some(async_filesystem_cancel::<F>),
                block_align,
            )
            .to_result()
        }
    }

    /// 'Piggyback' on FMOD file reading routines to capture data as it's read.
    ///
    /// This allows users to capture data as FMOD reads it,
    /// which may be useful for extracting the raw data that FMOD reads for hard to support sources (for example internet streams).
    ///
    /// Note: This function is not to replace FMOD's file system. For this functionality, see [`System::set_filesystem_sync`].
    pub fn attach_filesystem<F: FileSystemSync>(&self) -> Result<()> {
        unsafe {
            FMOD_System_AttachFileSystem(
                self.as_ptr(),
                Some(filesystem_open::<F>),
                Some(filesystem_close::<F>),
                Some(filesystem_read::<F>),
                Some(filesystem_seek::<F>),
            )
            .to_result()
        }
    }

    /// Detach the currently attached listener filesystem.
    pub fn detach_filesystem(&self) -> Result<()> {
        unsafe {
            FMOD_System_AttachFileSystem(self.as_ptr(), None, None, None, None).to_result()
        }
    }
}
