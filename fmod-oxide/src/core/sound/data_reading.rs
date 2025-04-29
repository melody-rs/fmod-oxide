// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::c_uint;

use fmod_sys::*;

use crate::{OpenState, Sound};

#[derive(Debug)]
pub struct SoundLock<'a> {
    sound: Sound,
    data: &'a mut [u8],
    extra: Option<&'a mut [u8]>,
}

impl SoundLock<'_> {
    /// The first part of the locked data.
    pub fn data(&self) -> &[u8] {
        self.data
    }

    /// The first part of the locked data.
    pub fn data_mut(&self) -> &[u8] {
        self.data
    }

    /// Second part of the locked data if the `offset` + `length` has exceeded the length of the sample buffer.
    pub fn extra(&self) -> Option<&[u8]> {
        match &self.extra {
            Some(extra) => Some(*extra),
            None => None,
        }
    }

    /// Second part of the locked data if the `offset` + `length` has exceeded the length of the sample buffer.
    pub fn extra_mut(&mut self) -> Option<&mut [u8]> {
        match &mut self.extra {
            Some(extra) => Some(*extra),
            None => None,
        }
    }
}

impl Drop for SoundLock<'_> {
    fn drop(&mut self) {
        let result = unsafe {
            let extra_ptr = self
                .extra
                .as_deref_mut()
                .map_or(std::ptr::null_mut(), <[u8]>::as_mut_ptr)
                .cast();
            let extra_len = self.extra.as_deref().map_or(0, <[u8]>::len) as c_uint;

            FMOD_Sound_Unlock(
                self.sound.inner.as_ptr(),
                self.data.as_mut_ptr().cast(),
                extra_ptr,
                self.data.len() as c_uint,
                extra_len,
            )
            .to_result()
        };
        if let Err(e) = result {
            eprintln!("FMOD_Sound_Unlock errored: {e}s");
        }
    }
}

impl Sound {
    /// Retrieves the state a sound is in after being opened with the non blocking flag, or the current state of the streaming buffer.
    ///
    /// When a sound is opened with `FMOD_NONBLOCKING`, it is opened and prepared in the background, or asynchronously.
    /// This allows the main application to execute without stalling on audio loads.
    /// This function will describe the state of the asynchronous load routine i.e. whether it has succeeded, failed or is still in progress.
    ///
    /// If 'starving' is true, then you will most likely hear a stuttering/repeating sound as the decode buffer loops on itself and replays old data.
    /// With the ability to detect stream starvation, muting the sound with `ChannelControl::setMute` will keep the stream quiet until it is not starving any more.
    ///
    /// #### Note: Always check [`OpenState`] to determine the state of the sound.
    /// Do not assume that if this function returns [`Ok`] then the sound has finished loading.
    pub fn get_open_state(&self) -> Result<(OpenState, c_uint, bool, bool)> {
        let mut open_state = 0;
        let mut percent_buffered = 0;
        let mut starving = FMOD_BOOL::FALSE;
        let mut disk_busy = FMOD_BOOL::FALSE;
        let error = unsafe {
            FMOD_Sound_GetOpenState(
                self.inner.as_ptr(),
                &raw mut open_state,
                &raw mut percent_buffered,
                &raw mut starving,
                &raw mut disk_busy,
            )
            .to_error()
        };

        let open_state = OpenState::try_from_ffi(open_state, error)?;
        let starving = starving.into();
        let disk_busy = disk_busy.into();
        Ok((open_state, percent_buffered, starving, disk_busy))
    }

    /// Gives access to a portion or all the sample data of a sound for direct manipulation.
    ///
    /// With this function you get access to the raw audio data.
    /// If the data is 8, 16, 24 or 32bit PCM data, mono or stereo data, you must take this into consideration when processing the data.
    /// See Sample Data for more information.
    ///
    /// If the sound is created with [`FMOD_CREATECOMPRESSEDSAMPLE`] the data retrieved will be the compressed bitstream.
    ///
    /// It is not possible to lock the following:
    /// - A parent sound containing subsounds. A parent sound has no audio data and [`FMOD_ERR_SUBSOUNDS`] will be returned.
    /// - A stream / sound created with [`FMOD_CREATESTREAM`]. [`FMOD_ERR_BADCOMMAND`] will be returned in this case.
    ///
    /// The names 'lock'/'unlock' are a legacy reference to older Operating System APIs that used to cause a mutex lock on the data,
    /// so that it could not be written to while the 'lock' was in place.
    /// This is no longer the case with FMOD and data can be 'locked' multiple times from different places/threads at once.
    ///
    /// # Safety
    ///
    /// While [`SoundLock`]'s lifetime is tied to `self`, it's a very loose coupling because [`Sound`] is [`Copy`]!
    /// If FMOD frees the memory pointed to by [`SoundLock`], it's insta UB.
    ///
    /// Don't call [`FMOD_Sound_Unlock`] with the pointers from [`SoundLock`]. [`SoundLock`] will do that for you when dropped.
    ///
    /// This function can hand out multiple mutable references to the same data if you aren't careful.
    /// FMOD doesn't perform any actual locking anymore! (see above!)
    pub unsafe fn lock(&self, offset: c_uint, length: c_uint) -> Result<SoundLock<'_>> {
        unsafe {
            let mut data = std::ptr::null_mut();
            let mut extra = std::ptr::null_mut();

            let mut data_len = 0;
            let mut extra_len = 0;

            FMOD_Sound_Lock(
                self.inner.as_ptr(),
                offset,
                length,
                &raw mut data,
                &raw mut extra,
                &raw mut data_len,
                &raw mut extra_len,
            )
            .to_result()?;

            let data = std::slice::from_raw_parts_mut(data.cast(), data_len as usize);
            let extra = if extra.is_null() {
                None
            } else {
                Some(std::slice::from_raw_parts_mut(
                    extra.cast(),
                    extra_len as usize,
                ))
            };

            Ok(SoundLock {
                sound: *self,
                data,
                extra,
            })
        }
    }

    /// This can be used for decoding data offline in small pieces (or big pieces), rather than playing and capturing it,
    /// or loading the whole file at once and having to [`Sound::lock`] / [`Sound::unlock`] the data.
    ///
    /// If too much data is read, it is possible [`FMOD_ERR_FILE_EOF`] will be returned, meaning it is out of data.
    /// The 'read' parameter will reflect this by returning a smaller number of bytes read than was requested.
    ///
    /// As a non streaming sound reads and decodes the whole file then closes it upon calling [`System::create_sound`],
    /// [`Sound::read_data`] will then not work because the file handle is closed. Use [`FMOD_OPENONLY`] to stop FMOD reading/decoding the file.
    /// If [`FMOD_OPENONLY`] flag is used when opening a sound, it will leave the file handle open,
    /// and FMOD will not read/decode any data internally, so the read cursor will stay at position 0.
    /// This will allow the user to read the data from the start.
    ///
    /// For streams, the streaming engine will decode a small chunk of data and this will advance the read cursor.
    /// You need to either use [`FMOD_OPENONLY`] to stop the stream pre-buffering or call [`Sound::seek_data`] to reset the read cursor back to the start of the file,
    /// otherwise it will appear as if the start of the stream is missing.
    /// [`Channel::setPosition`] will have the same result. These functions will flush the stream buffer and read in a chunk of audio internally.
    /// This is why if you want to read from an absolute position you should use [`Sound::seek_data`] and not the previously mentioned functions.
    ///
    /// If you are calling [`Sound::read_data`] and [`Sound::seek_data`] on a stream,
    /// information functions such as [`Channel::getPosition`] may give misleading results.
    /// Calling [`Channel::setPosition`] will cause the streaming engine to reset and flush the stream,
    /// leading to the time values returning to their correct position.
    ///
    /// # Safety
    ///
    /// If you call this from another stream callback, or any other thread besides the main thread,
    /// make sure to syncrhonize the callback with [`Sound::release`] in case the sound is still being read from while releasing.
    ///
    /// This function is thread safe to call from a stream callback or different thread as long as it doesnt conflict with a call to [`Sound::release`].
    pub unsafe fn read_data(&self, buffer: &mut [u8]) -> Result<c_uint> {
        unsafe {
            let mut read = 0;
            FMOD_Sound_ReadData(
                self.inner.as_ptr(),
                buffer.as_mut_ptr().cast(),
                buffer.len() as c_uint,
                &raw mut read,
            )
            .to_result()?;
            Ok(read)
        }
    }

    /// For use in conjunction with [`Sound::read_data`] and [`FMOD_OPENONLY`].
    ///
    /// For streaming sounds, if this function is called, it will advance the internal file pointer but not update the streaming engine.
    /// This can lead to de-synchronization of position information for the stream and audible playback.
    ///
    /// A stream can reset its stream buffer and position synchronization by calling [`Channel::setPosition`].
    /// This causes reset and flush of the stream buffer.
    pub fn seek_data(&self, pcm: c_uint) -> Result<()> {
        unsafe { FMOD_Sound_SeekData(self.inner.as_ptr(), pcm).to_result() }
    }
}
