// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod channel_group;
pub use channel_group::*;

mod sound_group;
pub use sound_group::*;

mod reverb_3d;
pub use reverb_3d::*;

mod channel;
pub use channel::*;

mod channel_control;
pub use channel_control::*;

mod geometry;
pub use geometry::*;

mod system;
pub use system::*;

mod sound;
pub use sound::*;

mod dsp;
pub use dsp::*;

mod dsp_connection;
pub use dsp_connection::*;

mod flags;
pub use flags::*;

mod enums;
pub use enums::*;

mod reverb_presets;
mod structs;
pub use structs::*;

/// Low level control over FMOD's debug logging.
pub mod debug;
/// Low level control over FMOD's filesystem access.
pub mod file;
/// Low level control over how FMOD allocates memory.
pub mod memory;
/// Low level control over FMOD's threads.
pub mod thread;

mod filesystem;
pub use filesystem::*;

mod helpers;
pub(crate) use helpers::*;
