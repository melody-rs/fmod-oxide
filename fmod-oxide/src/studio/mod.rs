// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![warn(missing_docs)]

use fmod_sys::*;
use std::{ffi::c_char, os::raw::c_int};

mod structs;

use lanyard::Utf8CString;
pub use structs::*;

mod flags;
pub use flags::*;

mod enums;
pub use enums::*;

mod bank;
pub use bank::*;

mod bus;
pub use bus::*;

mod system;
pub use system::*;

mod command_replay;
pub use command_replay::*;

mod event_description;
pub use event_description::*;

mod event_instance;
pub use event_instance::*;

mod vca;
pub use vca::*;

fn get_string_out_size(
    mut get_fn: impl FnMut(*mut c_char, c_int, *mut c_int) -> fmod_sys::FMOD_RESULT,
) -> fmod_sys::Result<Utf8CString> {
    let mut string_len = 0;

    match get_fn(std::ptr::null_mut(), 0, &raw mut string_len).to_error() {
        Some(err) if err != FMOD_RESULT::FMOD_ERR_TRUNCATED => return Err(err),
        _ => {}
    }

    let mut buf = vec![0u8; string_len as usize];
    let mut expected_string_len = 0;

    get_fn(
        buf.as_mut_ptr().cast(),
        string_len,
        &raw mut expected_string_len,
    )
    .to_result()?;

    debug_assert_eq!(string_len, expected_string_len);

    let string = unsafe { Utf8CString::from_utf8_with_nul_unchecked(buf) };
    Ok(string)
}

/// The required memory alignment of banks in user memory.
///
/// When using [`System::load_bank_pointer`] you must align the past slice to this alignment.
pub const LOAD_POINT_ALIGNMENT: usize = FMOD_STUDIO_LOAD_MEMORY_ALIGNMENT as _;
