// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::{c_int, c_void};

use fmod_sys::*;

use crate::studio::{AdvancedSettings, InitFlags, System};
use crate::{FmodResultExt, Result};

/// A builder for creating and initializing a [`System`].
///
/// Handles setting values that can only be set before initialization for you.
#[must_use]
#[derive(Debug)]
pub struct SystemBuilder {
    system: *mut FMOD_STUDIO_SYSTEM,
    core_builder: crate::SystemBuilder,
    sync_update: bool,
}

#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for SystemBuilder {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for SystemBuilder {}

impl SystemBuilder {
    /// Creates a new [`SystemBuilder`].
    ///
    /// # Safety
    ///
    /// Calling this function concurrently with any FMOD Studio API function (including this function) may cause undefined behavior.
    /// External synchronization must be used if calls to [`SystemBuilder::new`] or [`System::release`] could overlap other FMOD Studio API calls.
    /// All other FMOD Studio API functions are thread safe and may be called freely from any thread unless otherwise documented.
    pub unsafe fn new() -> Result<Self> {
        let mut system = std::ptr::null_mut();
        unsafe { FMOD_Studio_System_Create(&raw mut system, FMOD_VERSION).to_result()? };

        let mut core_system = std::ptr::null_mut();
        unsafe { FMOD_Studio_System_GetCoreSystem(system, &raw mut core_system).to_result()? };

        Ok(SystemBuilder {
            system,
            core_builder: crate::SystemBuilder {
                system: core_system,
                thread_unsafe: false,
            },
            sync_update: false,
        })
    }

    /// # Safety
    ///
    /// This function sets up FMOD Studio to run all commands on the calling thread, and FMOD Studio expects all calls to be issued from a single thread.
    ///
    /// This has the side effect of making *EVERY* Studio Struct in this crate `!Send` and `!Sync` *without* marking them as `!Send` and `!Sync`.
    /// This means that there are no handrails preventing you from using FMOD Studio across multiple threads, and you *must* ensure this yourself!
    #[cfg(not(feature = "thread-unsafe"))]
    pub unsafe fn synchronous_update(&mut self) {
        self.sync_update = true;
    }

    #[cfg(feature = "thread-unsafe")]
    pub fn synchronous_update(&mut self) {
        self.sync_update = true;
    }

    /// Sets advanced settings.
    pub fn settings(&mut self, settings: &AdvancedSettings) -> Result<&mut Self> {
        let mut settings = settings.into();
        // this function expects a pointer. maybe this is incorrect?
        unsafe {
            FMOD_Studio_System_SetAdvancedSettings(self.system, &raw mut settings).to_result()
        }?;
        Ok(self)
    }

    /// Builds the Studio System.
    ///
    /// The core system used by the studio system is initialized at the same time as the studio system.
    pub fn build(
        self,
        max_channels: c_int,
        studio_flags: InitFlags,
        flags: crate::InitFlags,
    ) -> Result<System> {
        unsafe {
            // we don't need
            self.build_with_extra_driver_data(
                max_channels,
                studio_flags,
                flags,
                std::ptr::null_mut(),
            )
        }
    }

    /// Returns the FMOD core `SystemBuilder`.
    ///
    /// This function only returns a `&mut` reference to prevent building the core `System` as building the studio `System` will handle that for you.
    pub fn core_builder(&mut self) -> &mut crate::SystemBuilder {
        &mut self.core_builder
    }

    /// # Safety
    ///
    /// See the FMOD docs explaining driver data for more safety information.
    pub unsafe fn build_with_extra_driver_data(
        self,
        max_channels: c_int,
        mut studio_flags: InitFlags,
        flags: crate::InitFlags,
        driver_data: *mut c_void,
    ) -> Result<System> {
        if self.sync_update {
            studio_flags.insert(InitFlags::SYNCHRONOUS_UPDATE);
        } else {
            #[cfg(not(feature = "thread-unsafe"))]
            studio_flags.remove(InitFlags::SYNCHRONOUS_UPDATE);
        }
        unsafe {
            FMOD_Studio_System_Initialize(
                self.system,
                max_channels,
                studio_flags.bits(),
                flags.bits(),
                driver_data,
            )
            .to_result()?;
            Ok(System::from_ffi(self.system))
        }
    }
}
