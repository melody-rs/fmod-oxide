// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::{Utf8CStr, Utf8CString};

pub(crate) fn get_string(
    mut string_fn: impl FnMut(&mut [u8]) -> FMOD_RESULT,
) -> Result<Utf8CString> {
    // Use stack-based buffer initially.
    let mut buffer = [0; 256];
    // Initial call to get the string.
    let mut result = string_fn(&mut buffer);

    match result.to_error() {
        // String fit in 256 bytes
        None => {
            let string = Utf8CStr::from_utf8_until_nul(&buffer).unwrap().to_cstring();
            return Ok(string);
        }
        // Didn't fit. Try using Vec
        Some(Error::Fmod(FMOD_RESULT::FMOD_ERR_TRUNCATED)) => {}
        Some(e) => return Err(e),
    }

    let mut buffer = vec![0u8; 256];
    // If the buffer is too small, resize it and try again.
    while let FMOD_RESULT::FMOD_ERR_TRUNCATED = result {
        buffer.resize(buffer.len() * 2, 0);
        result = string_fn(&mut buffer);
    }

    result.to_result()?;

    let string = Utf8CStr::from_utf8_until_nul(&buffer).unwrap().to_cstring();
    Ok(string)
}

pub(crate) fn string_from_utf16_le(utf16: &[u16]) -> String {
    let iter = utf16.iter().copied().map(u16::from_le);
    // we use char::decode_utf16 instead of String::from_utf16 because the latter would require us to collect into a Vec<u16> first
    char::decode_utf16(iter)
        .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
        .collect()
}

pub(crate) fn string_from_utf16_be(utf16: &[u16]) -> String {
    let iter = utf16.iter().copied().map(u16::from_be);
    // we use char::decode_utf16 instead of String::from_utf16 because the latter would require us to collect into a Vec<u16> first
    char::decode_utf16(iter)
        .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
        .collect()
}
