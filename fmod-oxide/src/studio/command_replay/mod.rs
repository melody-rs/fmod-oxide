// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ptr::NonNull;

use fmod_sys::*;

mod callback;
mod general;
mod playback;
mod query;
pub use callback::{CreateInstanceCallback, FrameCallback, LoadBankCallback};

/// The FMOD Studio command replay system allows API calls in a session to be recorded and later played back for debugging and performance purposes.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct CommandReplay {
    pub(crate) inner: NonNull<FMOD_STUDIO_COMMANDREPLAY>,
}

#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for CommandReplay {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for CommandReplay {}

impl CommandReplay {
    /// # Safety
    ///
    /// `value` must be a valid pointer either aquired from [`Self::as_ptr`] or FMOD.
    ///
    /// # Panics
    ///
    /// Panics if `value` is null.
    pub unsafe fn from_ffi(value: *mut FMOD_STUDIO_COMMANDREPLAY) -> Self {
        let inner = NonNull::new(value).unwrap();
        CommandReplay { inner }
    }

    /// Converts `self` into its raw representation.
    pub fn as_ptr(self) -> *mut FMOD_STUDIO_COMMANDREPLAY {
        self.inner.as_ptr()
    }
}

impl From<CommandReplay> for *mut FMOD_STUDIO_COMMANDREPLAY {
    fn from(value: CommandReplay) -> Self {
        value.inner.as_ptr()
    }
}
