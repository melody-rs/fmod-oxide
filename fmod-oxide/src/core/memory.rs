// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{FmodResultExt, Result};
use fmod_sys::*;
use std::ffi::{c_char, c_int, c_uint, c_void};

/// How you want FMOD to handle memory.
#[derive(PartialEq, Eq, Debug)]
pub enum MemoryType {
    /// Static block of memory for FMOD to manage.
    Pool(&'static mut [u8]),
    /// Custom allocator functions.
    Callback {
        /// Memory allocation callback compatible with ANSI malloc.
        alloc: unsafe extern "C" fn(
            size: c_uint,
            type_: FMOD_MEMORY_TYPE,
            sourcestr: *const c_char,
        ) -> *mut c_void,
        ///  Memory reallocation callback compatible with ANSI realloc.
        realloc: FMOD_MEMORY_REALLOC_CALLBACK,
        /// Memory free callback compatible with ANSI free.
        free: unsafe extern "C" fn(
            ptr: *mut c_void,
            type_: FMOD_MEMORY_TYPE,
            sourcestr: *const c_char,
        ),
    },
}

bitflags::bitflags! {
    /// Bitfields for memory allocation type being passed into FMOD memory callbacks.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct MemoryFlags: FMOD_MEMORY_TYPE {
        /// Standard memory.
      const NORMAL         = FMOD_MEMORY_NORMAL;
      /// Stream file buffer, size controllable with [`System::setStreamBufferSize`].
      const STREAM_FILE    = FMOD_MEMORY_STREAM_FILE;
      /// Stream decode buffer, size controllable with [`FMOD_CREATESOUNDEXINFO::decodebuffersize`].
      const STREAM_DECODE  = FMOD_MEMORY_STREAM_DECODE;
      /// Sample data buffer. Raw audio data, usually PCM/MPEG/ADPCM/XMA data.
      const SAMPLEDATA     = FMOD_MEMORY_SAMPLEDATA;
      /// Deprecated.
      #[deprecated]
      const DSP_BUFFER     = FMOD_MEMORY_DSP_BUFFER;
      /// Memory allocated by a third party plugin.
      const PLUGIN         = FMOD_MEMORY_PLUGIN;
      /// Persistent memory. Memory will be freed when [`System::release`] is called.
      const PERSISTENT     = FMOD_MEMORY_PERSISTENT;
      /// Mask specifying all memory types.
      const ALL            = FMOD_MEMORY_ALL;
    }
}

impl From<FMOD_MEMORY_TYPE> for MemoryFlags {
    fn from(value: FMOD_MEMORY_TYPE) -> Self {
        MemoryFlags::from_bits_truncate(value)
    }
}

impl From<MemoryFlags> for FMOD_MEMORY_TYPE {
    fn from(value: MemoryFlags) -> Self {
        value.bits()
    }
}

/// Specifies a method for FMOD to allocate and free memory, either through user supplied callbacks or through a user supplied memory buffer with a fixed size.
///
/// # Safety
///
/// This function must be called before any FMOD System object is created.
///
/// If [`MemoryType::Callback::alloc`] and [`MemoryType::Callback::free`] are provided without [`MemoryType::Callback::realloc`]
/// the reallocation is implemented via an allocation of the new size, copy from old address to new, then a free of the old address.
///
/// Callback implementations must be thread safe.
///
/// If you specify a fixed size pool that is too small, FMOD will return [`FMOD_RESULT::FMOD_ERR_MEMORY`] when the limit of the fixed size pool is exceeded.
/// At this point, it's possible that FMOD may become unstable. To maintain stability, do not allow FMOD to run out of memory.
/// To find out the required fixed size call [`initialize`] with an overly large pool size (or no pool) and find out the maximum RAM usage at any one time with [`get_stats`].
/// The size of the pool is limited to [`c_int::MAX`].
pub unsafe fn initialize(memory_type: MemoryType, flags: MemoryFlags) -> Result<()> {
    match memory_type {
        MemoryType::Pool(pool) => unsafe {
            FMOD_Memory_Initialize(
                pool.as_mut_ptr().cast(),
                pool.len() as c_int,
                None,
                None,
                None,
                flags.into(),
            )
            .to_result()
        },
        MemoryType::Callback {
            alloc,
            realloc,
            free,
        } => unsafe {
            FMOD_Memory_Initialize(
                std::ptr::null_mut(),
                0,
                Some(alloc),
                realloc,
                Some(free),
                flags.into(),
            )
            .to_result()
        },
    }
}

/// Returns information on the memory usage of FMOD.
///
/// This information is byte accurate and counts all allocs and frees internally.
/// This is useful for determining a fixed memory size to make FMOD work within for fixed memory machines such as consoles.
///
/// Note that if using [`initialize`], the memory usage will be slightly higher than without it, as FMOD has to have a small amount of memory overhead to manage the available memory.
pub fn get_stats(blocking: bool) -> Result<(c_int, c_int)> {
    let mut current = 0;
    let mut max = 0;
    unsafe {
        FMOD_Memory_GetStats(&raw mut current, &raw mut max, blocking.into()).to_result()?;
    }
    Ok((current, max))
}
