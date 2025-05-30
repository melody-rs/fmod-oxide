// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CString;

use crate::{FmodResultExt, Result};
use std::ffi::{c_char, c_int};

/// Specify the destination of log output when using the logging version of FMOD.
#[derive(PartialEq, Eq, Debug)]
pub enum DebugMode {
    /// Default log location per platform, i.e. Visual Studio output window, stderr, `LogCat`, etc.
    TTY,
    /// Write log to specified file path.
    File(Utf8CString),
    /// Call specified callback with log information.
    Callback(
        unsafe extern "C" fn(
            FMOD_DEBUG_FLAGS,
            *const c_char,
            c_int,
            *const c_char,
            *const c_char,
        ) -> FMOD_RESULT,
    ),
}

bitflags::bitflags! {
    /// Specify the requested information to be output when using the logging version of FMOD.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct DebugFlags: FMOD_DEBUG_FLAGS {
        /// Disable all messages.
        const NONE = FMOD_DEBUG_LEVEL_NONE;
        /// Enable only error messages.
        const ERROR = FMOD_DEBUG_LEVEL_ERROR;
        /// Enable warning and error messages.
        const WARNING = FMOD_DEBUG_LEVEL_WARNING;
        /// Enable informational, warning and error messages (default).
        const LOG = FMOD_DEBUG_LEVEL_LOG;
        /// Verbose logging for memory operations, only use this if you are debugging a memory related issue.
        const MEMORY = FMOD_DEBUG_TYPE_MEMORY;
        /// Verbose logging for file access, only use this if you are debugging a file related issue.
        const FILE = FMOD_DEBUG_TYPE_FILE;
        /// Verbose logging for codec initialization, only use this if you are debugging a codec related issue.
        const CODEC = FMOD_DEBUG_TYPE_CODEC;
        /// Verbose logging for internal errors, use this for tracking the origin of error codes.
        const TRACE = FMOD_DEBUG_TYPE_TRACE;
        /// Display the time stamp of the log message in milliseconds.
        const DISPLAY_TIMESTAMPS = FMOD_DEBUG_DISPLAY_TIMESTAMPS;
        /// Display the source code file and line number for where the message originated.
        const DISPLAY_LINENUMBERS = FMOD_DEBUG_DISPLAY_LINENUMBERS;
        /// Display the thread ID of the calling function that generated the message.
        const DISPLAY_THREAD = FMOD_DEBUG_DISPLAY_THREAD;
    }
}

impl From<FMOD_DEBUG_FLAGS> for DebugFlags {
    fn from(value: FMOD_DEBUG_FLAGS) -> Self {
        DebugFlags::from_bits_truncate(value)
    }
}

impl From<DebugFlags> for FMOD_DEBUG_FLAGS {
    fn from(value: DebugFlags) -> Self {
        value.bits()
    }
}

/// Specify the level and delivery method of log messages when using the logging version of FMOD.
///
/// This function will return [`FMOD_RESULT::FMOD_ERR_UNSUPPORTED`] when using the non-logging (release) versions of FMOD.
/// The logging version of FMOD can be recognized by the 'L' suffix in the library name, fmodL.dll or libfmodL.so for instance.
///
/// By default this crate links against non-logging versions of FMOD in release builds.
/// This behaviour can be changed with the "fmod-sys/force-debug" feature.
///
/// Note that:
///     [`DebugFlags::LOG`] produces informational, warning and error messages.
///     [`DebugFlags::WARNING`] produces warnings and error messages.
///     [`DebugFlags::ERROR`] produces error messages only.
pub fn initialize(flags: DebugFlags, mode: DebugMode) -> Result<()> {
    match mode {
        DebugMode::TTY => unsafe {
            FMOD_Debug_Initialize(flags.into(), FMOD_DEBUG_MODE_TTY, None, std::ptr::null())
                .to_result()
        },
        DebugMode::File(file) => unsafe {
            FMOD_Debug_Initialize(flags.into(), FMOD_DEBUG_MODE_FILE, None, file.as_ptr())
                .to_result()
        },
        DebugMode::Callback(c) => unsafe {
            FMOD_Debug_Initialize(
                flags.into(),
                FMOD_DEBUG_MODE_CALLBACK,
                Some(c),
                std::ptr::null(),
            )
            .to_result()
        },
    }
}
