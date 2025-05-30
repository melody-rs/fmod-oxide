// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;

use crate::studio::CommandReplay;
use crate::{FmodResultExt, Result};

impl CommandReplay {
    /// Releases the command replay.
    pub fn release(&self) -> Result<()> {
        unsafe { FMOD_Studio_CommandReplay_Release(self.inner.as_ptr()).to_result() }
    }
}
