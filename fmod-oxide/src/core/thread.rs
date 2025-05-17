// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{FmodResultExt, Result};
use crate::{ThreadAffinity, ThreadType};
use fmod_sys::*;

/// Scheduling priority to assign a given thread to.
pub mod priority {
    use fmod_sys::*;

    /// Lower bound of platform specific priority range.
    pub const PLATFORM_MIN: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_PLATFORM_MIN;
    /// Upper bound of platform specific priority range.
    pub const PLATFORM_MAX: FMOD_THREAD_PRIORITY =
        FMOD_THREAD_PRIORITY_PLATFORM_MAX as FMOD_THREAD_PRIORITY; // no idea why this is u32 (32768 fits in an i32)
    /// For a given thread use the default listed below, i.e. [`FMOD_THREAD_TYPE_MIXER`] uses [`FMOD_THREAD_PRIORITY_MIXER`].
    pub const DEFAULT: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_DEFAULT;
    /// Low platform agnostic priority.
    pub const LOW: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_LOW;
    /// Medium platform agnostic priority.
    pub const MEDIUM: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_MEDIUM;
    /// High platform agnostic priority.
    pub const HIGH: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_HIGH;
    /// Very high platform agnostic priority.
    pub const VERY_HIGH: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_VERY_HIGH;
    /// Extreme platform agnostic priority.
    pub const EXTREME: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_EXTREME;
    /// Critical platform agnostic priority.
    pub const CRITICAL: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_CRITICAL;
    /// Default priority for [`FMOD_THREAD_TYPE_MIXER`].
    pub const MIXER: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_MIXER;
    /// Default priority for [`FMOD_THREAD_TYPE_FEEDER`].
    pub const FEEDER: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_FEEDER;
    /// Default priority for [`FMOD_THREAD_TYPE_STREAM`].
    pub const STREAM: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_STREAM;
    /// Default priority for [`FMOD_THREAD_TYPE_FILE`].
    pub const FILE: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_FILE;
    /// Default priority for [`FMOD_THREAD_TYPE_NONBLOCKING`].
    pub const NONBLOCKING: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_NONBLOCKING;
    /// Default priority for [`FMOD_THREAD_TYPE_RECORD`].
    pub const RECORD: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_RECORD;
    /// Default priority for [`FMOD_THREAD_TYPE_GEOMETRY`].
    pub const GEOMETRY: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_GEOMETRY;
    /// Default priority for [`FMOD_THREAD_TYPE_PROFILER`].
    pub const PROFILER: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_PROFILER;
    /// Default priority for [`FMOD_THREAD_TYPE_STUDIO_UPDATE`].
    pub const STUDIO_UPDATE: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_STUDIO_UPDATE;
    /// Default priority for [`FMOD_THREAD_TYPE_STUDIO_LOAD_BANK`].
    pub const STUDIO_LOAD_BANK: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_STUDIO_LOAD_BANK;
    /// Default priority for [`FMOD_THREAD_TYPE_STUDIO_LOAD_SAMPLE`].
    pub const STUDIO_LOAD_SAMPLE: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_STUDIO_LOAD_SAMPLE;
    /// Default priority for [`FMOD_THREAD_TYPE_CONVOLUTION1`].
    pub const CONVOLUTION_1: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_CONVOLUTION1;
    /// Default priority for [`FMOD_THREAD_TYPE_CONVOLUTION2`].
    pub const CONVOLUTION_2: FMOD_THREAD_PRIORITY = FMOD_THREAD_PRIORITY_CONVOLUTION2;
}

/// Stack space available to the given thread.
pub mod stack_size {
    use fmod_sys::*;
    /// For a given thread use the default listed below, i.e. [`FMOD_THREAD_TYPE_MIXER`] uses [`FMOD_THREAD_STACK_SIZE_MIXER`].
    pub const DEFAULT: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_DEFAULT;
    /// Default stack size for [`FMOD_THREAD_TYPE_MIXER`].
    pub const MIXER: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_MIXER;
    /// Default stack size for [`FMOD_THREAD_TYPE_MIXER`].
    pub const FEEDER: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_FEEDER;
    /// Default stack size for [`FMOD_THREAD_TYPE_STREAM`].
    pub const STREAM: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_STREAM;
    /// Default stack size for [`FMOD_THREAD_TYPE_FILE`].
    pub const FILE: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_FILE;
    /// Default stack size for [`FMOD_THREAD_TYPE_NONBLOCKING`].
    pub const NONBLOCKING: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_NONBLOCKING;
    /// Default stack size for [`FMOD_THREAD_TYPE_RECORD`].
    pub const RECORD: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_RECORD;
    /// Default stack size for [`FMOD_THREAD_TYPE_GEOMETRY`].
    pub const GEOMETRY: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_GEOMETRY;
    /// Default stack size for [`FMOD_THREAD_TYPE_PROFILER`].
    pub const PROFILER: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_PROFILER;
    /// Default stack size for [`FMOD_THREAD_TYPE_STUDIO_UPDATE`].
    pub const STUDIO_UPDATE: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_STUDIO_UPDATE;
    /// Default stack size for [`FMOD_THREAD_TYPE_STUDIO_LOAD_BANK`].
    pub const STUDIO_LOAD_BANK: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_STUDIO_LOAD_BANK;
    /// Default stack size for [`FMOD_THREAD_TYPE_STUDIO_LOAD_SAMPLE`].
    pub const STUDIO_LOAD_SAMPLE: FMOD_THREAD_STACK_SIZE =
        FMOD_THREAD_STACK_SIZE_STUDIO_LOAD_SAMPLE;
    /// Default stack size for [`FMOD_THREAD_TYPE_CONVOLUTION1`].
    pub const CONVOLUTION_1: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_CONVOLUTION1;
    /// Default stack size for [`FMOD_THREAD_TYPE_CONVOLUTION2`].
    pub const CONVOLUTION_2: FMOD_THREAD_STACK_SIZE = FMOD_THREAD_STACK_SIZE_CONVOLUTION2;
}

/// Specify the affinity, priority and stack size for all FMOD created threads.
///
/// You must call this function for the chosen thread before that thread is created for the settings to take effect.
///
/// Affinity can be specified using one (or more) of the [`ThreadAffinity`] constants or by providing the bits explicitly, i.e. (1<<3) for logical core three (core affinity is zero based).
/// See platform documentation for details on the available cores for a given device.
///
/// Priority can be specified using one of the [`FMOD_THREAD_PRIORITY`] constants or by providing the value explicitly, i.e. (-2) for the lowest thread priority on Windows.
/// See platform documentation for details on the available priority values for a given operating system.
///
/// Stack size can be specified explicitly, however for each thread you should provide a size equal to or larger than the expected default or risk causing a stack overflow at runtime.
pub fn set_attributes(
    kind: ThreadType,
    affinity: ThreadAffinity,
    priority: FMOD_THREAD_PRIORITY,
    stack_size: FMOD_THREAD_STACK_SIZE,
) -> Result<()> {
    unsafe {
        FMOD_Thread_SetAttributes(kind.into(), affinity.into(), priority, stack_size).to_result()
    }
}
