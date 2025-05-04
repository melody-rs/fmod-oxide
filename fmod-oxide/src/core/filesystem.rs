// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

use lanyard::Utf8CStr;
use std::ffi::{c_char, c_int, c_uint, c_void};

// I was lost on how to do this for a while, so I took some pointers from https://github.com/CAD97/fmod-rs/blob/main/crates/fmod-rs/src/core/common/file.rs#L181
// It's not copied verbatim, I made some different design choices (like opting to make handle be a *mut c_void instead)
// for similar reasons to this crate not handling userdata.
// This is such a power user feature that I'm not sure it's worth hiding away most of the implementation details

// TODO test and validate my assumptions are correct

pub type Handle = *mut c_void;

#[derive(Debug)]
pub struct FileInfo {
    pub handle: Handle,
    pub file_size: c_uint,
}

pub trait FileSystem {
    fn open(name: &Utf8CStr, userdata: *mut c_void) -> Result<FileInfo>;

    fn close(handle: Handle, userdata: *mut c_void) -> Result<()>;
}

#[derive(Debug)]
pub struct FileBuffer<'a> {
    buffer: &'a mut [u8],
    written: &'a mut u32,
}

impl FileBuffer<'_> {
    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    pub fn written(&self) -> u32 {
        *self.written
    }

    pub fn is_full(&self) -> bool {
        self.written() == self.capacity() as u32
    }
}

impl std::io::Write for FileBuffer<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let unwritten_region = &mut self.buffer[*self.written as usize..];
        let len = buf.len().min(unwritten_region.len());
        self.buffer[..len].copy_from_slice(&buf[..len]);
        *self.written += len as u32;
        Ok(len)
    }

    // no-op
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub trait FileSystemSync: FileSystem {
    fn read(handle: Handle, userdata: *mut c_void, buffer: FileBuffer<'_>) -> Result<()>;

    fn seek(handle: Handle, userdata: *mut c_void, position: c_uint) -> Result<()>;
}

#[derive(Debug)]
pub struct AsyncReadInfo {
    raw: *mut FMOD_ASYNCREADINFO,
}

impl AsyncReadInfo {
    pub fn handle(&self) -> Handle {
        unsafe { *self.raw }.handle
    }

    pub fn offset(&self) -> c_uint {
        unsafe { *self.raw }.offset
    }

    pub fn size(&self) -> c_uint {
        unsafe { *self.raw }.sizebytes
    }

    pub fn priority(&self) -> c_int {
        unsafe { *self.raw }.priority
    }

    pub fn userdata(&self) -> *mut c_void {
        unsafe { *self.raw }.userdata
    }

    pub fn raw(&self) -> *mut FMOD_ASYNCREADINFO {
        self.raw
    }
    pub fn written(&self) -> c_uint {
        unsafe { *self.raw }.bytesread
    }

    // Normally this would be really unsafe because FMOD hands out the same *mut FMOD_ASYNCREADINFO to `read()` and `cancel()`.
    // AsyncCancelInfo doesn't support accessing the buffer, so this should be safe.
    pub fn buffer(&mut self) -> FileBuffer<'_> {
        let ptr = unsafe { *self.raw }.buffer;
        let len = self.size();

        let buffer = unsafe { std::slice::from_raw_parts_mut(ptr.cast(), len as usize) };
        let written = &mut unsafe { &mut *self.raw }.bytesread;
        FileBuffer { buffer, written }
    }

    /// If [`AsyncReadInfo::written`] != [`AsyncReadInfo::size`] this function will send an [`FMOD_ERR_FILE_EOF`] for you.
    ///
    /// # Safety
    ///
    /// If you have a [`AsyncCancelInfo`] with the same raw pointer, it is immediately invalid after calling this function.
    // I *really* don't like taking a result like this, but I can't think of another way...
    pub unsafe fn finish(self, result: Result<()>) {
        let mut fmod_result = result.into();
        if fmod_result == FMOD_RESULT::FMOD_OK && self.written() < self.size() {
            fmod_result = FMOD_RESULT::FMOD_ERR_FILE_EOF;
        }
        // Should never be null
        unsafe { (*self.raw).done.unwrap_unchecked()(self.raw, fmod_result) }
    }
}

#[derive(Debug)]
pub struct AsyncCancelInfo {
    raw: *mut FMOD_ASYNCREADINFO,
}

impl AsyncCancelInfo {
    pub fn handle(&self) -> Handle {
        unsafe { *self.raw }.handle
    }

    pub fn offset(&self) -> c_uint {
        unsafe { *self.raw }.offset
    }

    pub fn size(&self) -> c_uint {
        unsafe { *self.raw }.sizebytes
    }

    pub fn priority(&self) -> c_int {
        unsafe { *self.raw }.priority
    }

    pub fn userdata(&self) -> *mut c_void {
        unsafe { *self.raw }.userdata
    }

    pub fn raw(&self) -> *mut FMOD_ASYNCREADINFO {
        self.raw
    }
}

/// # Safety
///
/// This trait is marked as unsafe because a correct implementation of [`FileSystemAsync`] is hard to get right.
/// I'd suggest reading up on the FMOD documentation to get a better idea of how to write this.
pub unsafe trait FileSystemAsync: FileSystem {
    fn read(info: AsyncReadInfo, userdata: *mut c_void) -> Result<()>;

    fn cancel(info: AsyncCancelInfo, userdata: *mut c_void) -> Result<()>;
}

pub(crate) unsafe extern "C" fn filesystem_open<F: FileSystem>(
    name: *const c_char,
    raw_filesize: *mut c_uint,
    raw_handle: *mut Handle,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    let name = unsafe { Utf8CStr::from_ptr_unchecked(name) };
    let FileInfo { handle, file_size } = match F::open(name, userdata) {
        Ok(h) => h,
        Err(e) => return e.into(),
    };
    unsafe {
        *raw_filesize = file_size;
        *raw_handle = handle;
    }
    FMOD_RESULT::FMOD_OK
}

pub(crate) unsafe extern "C" fn filesystem_close<F: FileSystem>(
    handle: Handle,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    F::close(handle, userdata).into()
}

pub(crate) unsafe extern "C" fn filesystem_read<F: FileSystemSync>(
    handle: Handle,
    buffer: *mut c_void,
    size_bytes: c_uint,
    bytes_read: *mut c_uint,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    let buffer = unsafe {
        FileBuffer {
            buffer: std::slice::from_raw_parts_mut(buffer.cast(), size_bytes as usize),
            written: &mut *bytes_read,
        }
    };
    if let Err(e) = F::read(handle, userdata, buffer) {
        return e.into();
    }

    if unsafe { *bytes_read } < size_bytes {
        FMOD_RESULT::FMOD_ERR_FILE_EOF
    } else {
        FMOD_RESULT::FMOD_OK
    }
}

pub(crate) unsafe extern "C" fn filesystem_seek<F: FileSystemSync>(
    handle: Handle,
    pos: c_uint,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    F::seek(handle, userdata, pos).into()
}

pub(crate) unsafe extern "C" fn async_filesystem_read<F: FileSystemAsync>(
    raw: *mut FMOD_ASYNCREADINFO,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    F::read(AsyncReadInfo { raw }, userdata).into()
}

pub(crate) unsafe extern "C" fn async_filesystem_cancel<F: FileSystemAsync>(
    raw: *mut FMOD_ASYNCREADINFO,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    F::cancel(AsyncCancelInfo { raw }, userdata).into()
}
