// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

use crate::{FmodResultExt, Result};
use lanyard::Utf8CStr;
use std::ffi::{c_char, c_int, c_uint, c_void};

// I was lost on how to do this for a while, so I took some pointers from https://github.com/CAD97/fmod-rs/blob/main/crates/fmod-rs/src/core/common/file.rs#L181
// It's not copied verbatim, I made some different design choices (like opting to make handle be a *mut c_void instead)
// for similar reasons to this crate not handling userdata.
// This is such a power user feature that I'm not sure it's worth hiding away most of the implementation details

// TODO test and validate my assumptions are correct

/// The base trait for all filesystems.
pub trait FileSystem {
    /// Callback for opening a file.
    ///
    /// Return the appropriate error code such as [`FMOD_ERR_FILE_NOTFOUND`] if the file fails to open.
    /// If the callback is from [`System::attachFileSystem`], then the return value is ignored.
    fn open(name: &Utf8CStr, userdata: *mut c_void) -> Result<(*mut c_void, c_uint)>;

    /// Callback for closing a file.
    ///
    /// Close any user created file handle and perform any cleanup necessary for the file here.
    /// If the callback is from [`System::attachFileSystem`], then the return value is ignored.
    fn close(handle: *mut c_void, userdata: *mut c_void) -> Result<()>;
}

/// A mutable file buffer.
///
/// It's a lot like [`std::io::Cursor`].
#[derive(Debug)]
pub struct FileBuffer<'a> {
    buffer: &'a mut [u8],
    written: &'a mut u32,
}

impl FileBuffer<'_> {
    /// The capacity of this file buffer.
    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    /// The total amount of bytes written.
    pub fn written(&self) -> u32 {
        *self.written
    }

    /// Returns true if [`Self::written`] == [`Self::capacity`].
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

/// A synchronous filesystem.
///
/// You should prefer implementing this trait over [`FileSystemAsync`]- it's a lot easier to do.
pub trait FileSystemSync: FileSystem {
    /// Callback for reading from a file.
    ///
    /// If the callback is from [`System::attachFileSystem`], then the return value is ignored.
    ///
    /// If there is not enough data to read the requested number of bytes,
    /// return fewer bytes in the bytesread parameter and and return [`FMOD_ERR_FILE_EOF`].
    fn read(handle: *mut c_void, userdata: *mut c_void, buffer: FileBuffer<'_>) -> Result<()>;

    /// Callback for seeking within a file.
    ///
    /// If the callback is from [`System::attachFileSystem`], then the return value is ignored.
    fn seek(handle: *mut c_void, userdata: *mut c_void, position: c_uint) -> Result<()>;
}

/// Information about a single asynchronous file operation.
#[derive(Debug)]
pub struct AsyncReadInfo {
    raw: *mut FMOD_ASYNCREADINFO,
}

impl AsyncReadInfo {
    /// File handle that was returned in [`FileSystem::open`].
    pub fn handle(&self) -> *mut c_void {
        unsafe { *self.raw }.handle
    }

    /// Offset within the file where the read operation should occur.
    pub fn offset(&self) -> c_uint {
        unsafe { *self.raw }.offset
    }

    /// Number of bytes to read.
    pub fn size(&self) -> c_uint {
        unsafe { *self.raw }.sizebytes
    }

    /// Priority hint for how quickly this operation should be serviced where 0 represents low importance and 100 represents extreme importance.
    /// This could be used to prioritize the read order of a file job queue for example.
    /// FMOD decides the importance of the read based on if it could degrade audio or not.
    pub fn priority(&self) -> c_int {
        unsafe { *self.raw }.priority
    }

    /// User value associated with this async operation, passed to [`FileSystemAsync::cancel`].
    pub fn userdata(&self) -> *mut c_void {
        unsafe { *self.raw }.userdata
    }

    /// Set the user value associated with this async operation.
    ///
    /// # Safety
    ///
    /// You cannot call this while a [`AsyncCancelInfo`] with the same raw pointer is live.
    // FIXME: make this safe somehow?
    pub unsafe fn set_userdata(&mut self, userdata: *mut c_void) {
        unsafe { *self.raw }.userdata = userdata;
    }

    /// Get the raw pointer associated with this [`AsyncReadInfo`].
    pub fn raw(&self) -> *mut FMOD_ASYNCREADINFO {
        self.raw
    }

    /// Number of bytes currently read.
    pub fn written(&self) -> c_uint {
        unsafe { *self.raw }.bytesread
    }

    /// Get the [`FileBuffer`] associated with this [`AsyncReadInfo`].
    // Normally this would be really unsafe because FMOD hands out the same *mut FMOD_ASYNCREADINFO to `read()` and `cancel()`.
    // AsyncCancelInfo doesn't support accessing the buffer, so this should be safe.
    pub fn buffer(&mut self) -> FileBuffer<'_> {
        let ptr = unsafe { *self.raw }.buffer;
        let len = self.size();

        let buffer = unsafe { std::slice::from_raw_parts_mut(ptr.cast(), len as usize) };
        let written = &mut unsafe { &mut *self.raw }.bytesread;
        FileBuffer { buffer, written }
    }

    /// Signal the async read is done.
    ///
    /// If [`AsyncReadInfo::written`] != [`AsyncReadInfo::size`] this function will send an [`FMOD_ERR_FILE_EOF`] for you.
    ///
    /// # Safety
    ///
    /// If you have a [`AsyncCancelInfo`] with the same raw pointer, it is immediately invalid after calling this function.
    // I *really* don't like taking a result like this, but I can't think of another way...
    pub unsafe fn finish(self, result: Result<()>) {
        let mut fmod_result = FMOD_RESULT::from_result(result);
        if fmod_result == FMOD_RESULT::FMOD_OK && self.written() < self.size() {
            fmod_result = FMOD_RESULT::FMOD_ERR_FILE_EOF;
        }
        // Should never be null
        unsafe { (*self.raw).done.unwrap_unchecked()(self.raw, fmod_result) }
    }
}

/// Information about cancelling a asynchronous file operation.
#[derive(Debug)]
pub struct AsyncCancelInfo {
    raw: *mut FMOD_ASYNCREADINFO,
}

impl AsyncCancelInfo {
    /// File handle that was returned in [`FileSystem::open`].
    pub fn handle(&self) -> *mut c_void {
        unsafe { *self.raw }.handle
    }

    /// Offset within the file where the read operation should occur.
    pub fn offset(&self) -> c_uint {
        unsafe { *self.raw }.offset
    }

    /// Number of bytes to read.
    pub fn size(&self) -> c_uint {
        unsafe { *self.raw }.sizebytes
    }

    /// Priority hint for how quickly this operation should be serviced where 0 represents low importance and 100 represents extreme importance.
    /// This could be used to prioritize the read order of a file job queue for example.
    /// FMOD decides the importance of the read based on if it could degrade audio or not.
    pub fn priority(&self) -> c_int {
        unsafe { *self.raw }.priority
    }

    /// User value associated with this async operation, passed to [`FileSystemAsync::cancel`].
    pub fn userdata(&self) -> *mut c_void {
        unsafe { *self.raw }.userdata
    }

    /// Get the raw pointer associated with this [`AsyncCancelInfo`].
    pub fn raw(&self) -> *mut FMOD_ASYNCREADINFO {
        self.raw
    }
}

/// An async filesystem.
///
/// # Safety
///
/// This trait is marked as unsafe because a correct implementation of [`FileSystemAsync`] is hard to get right.
/// I'd suggest reading up on the FMOD documentation to get a better idea of how to write this.
pub unsafe trait FileSystemAsync: FileSystem {
    /// Callback for reading from a file asynchronously.
    ///
    /// This callback allows you to accept a file I/O request without servicing it immediately.
    ///
    /// The callback can queue or store the [`AsyncReadInfo`],
    ///  so that a 'servicing routine' can read the data and mark the job as done.
    ///
    /// Marking an asynchronous job as 'done' outside of this callback can be done by calling [`AsyncReadInfo::finish`] with the file read result as a parameter.
    ///
    /// If the servicing routine is processed in the same thread as the thread that invokes this callback
    /// (for example the thread that calls [`System::createSound`] or[`System::createStream`]),
    /// a deadlock will occur because while [`System::createSound`] or [`System::createStream`] waits for the file data,
    /// the servicing routine in the main thread won't be able to execute.
    ///
    /// This typically means an outside servicing routine should typically be run in a separate thread.
    ///
    /// The read request can be queued or stored and this callback can return immediately with [`FMOD_OK`].
    /// Returning an error at this point will cause FMOD to stop what it was doing and return back to the caller.
    /// If it is from FMOD's stream thread, the stream will typically stop.
    fn read(info: AsyncReadInfo, userdata: *mut c_void) -> Result<()>;

    /// Callback for cancelling a pending asynchronous read.
    ///
    /// This callback is called to stop/release or shut down the resource that is holding the file,
    /// for example: releasing a Sound stream.
    ///
    /// Before returning from this callback the implementation must ensure that all references to info are relinquished.
    fn cancel(info: AsyncCancelInfo, userdata: *mut c_void) -> Result<()>;
}

pub(crate) unsafe extern "C" fn filesystem_open<F: FileSystem>(
    name: *const c_char,
    raw_filesize: *mut c_uint,
    raw_handle: *mut *mut c_void,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    let name = unsafe { Utf8CStr::from_ptr_unchecked(name) };
    let (handle, file_size) = match F::open(name, userdata) {
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
    handle: *mut c_void,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    let result = F::close(handle, userdata);
    FMOD_RESULT::from_result(result)
}

pub(crate) unsafe extern "C" fn filesystem_read<F: FileSystemSync>(
    handle: *mut c_void,
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
    handle: *mut c_void,
    pos: c_uint,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    let result = F::seek(handle, userdata, pos);
    FMOD_RESULT::from_result(result)
}

pub(crate) unsafe extern "C" fn async_filesystem_read<F: FileSystemAsync>(
    raw: *mut FMOD_ASYNCREADINFO,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    let result = F::read(AsyncReadInfo { raw }, userdata);
    FMOD_RESULT::from_result(result)
}

pub(crate) unsafe extern "C" fn async_filesystem_cancel<F: FileSystemAsync>(
    raw: *mut FMOD_ASYNCREADINFO,
    userdata: *mut c_void,
) -> FMOD_RESULT {
    let result = F::cancel(AsyncCancelInfo { raw }, userdata);
    FMOD_RESULT::from_result(result)
}
