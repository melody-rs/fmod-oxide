// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ptr::NonNull;
use std::{ffi::c_int, mem::MaybeUninit};

use crate::Guid;
use crate::studio::{Bank, Bus, EventDescription, Vca, get_string_out_size};
use fmod_sys::*;
use lanyard::Utf8CString;

impl Bank {
    /// Retrieves the number of buses in the bank.
    pub fn bus_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_Bank_GetBusCount(self.inner.as_ptr(), &raw mut count).to_result()?;
        }
        Ok(count)
    }

    /// Retrieves a list of the buses in the bank.
    pub fn get_bus_list(&self) -> Result<Vec<Bus>> {
        let expected_count = self.bus_count()?;
        let mut count = 0;
        let mut list = vec![
            Bus {
                inner: NonNull::dangling(), // we can't store a *null* NonNull pointer so we use NonNull::dangling instead
            };
            expected_count as usize
        ];

        unsafe {
            FMOD_Studio_Bank_GetBusList(
                self.inner.as_ptr(),
                // bus is repr transparent and has the same layout as *mut FMOD_STUDIO_BUS, so this cast is ok
                list.as_mut_ptr().cast::<*mut FMOD_STUDIO_BUS>(),
                list.capacity() as c_int,
                &raw mut count,
            )
            .to_result()?;

            debug_assert_eq!(count, expected_count);

            Ok(list)
        }
    }

    /// Retrives the number of event descriptions in the bank.
    ///
    /// This function counts the events which were added to the bank by the sound designer.
    /// The bank may contain additional events which are referenced by event instruments but were not added to the bank, and those referenced events are not counted.
    pub fn event_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_Bank_GetEventCount(self.inner.as_ptr(), &raw mut count).to_result()?;
        }
        Ok(count)
    }

    /// Retrieves a list of the event descriptions in the bank.
    ///
    /// This function counts the events which were added to the bank by the sound designer.
    /// The bank may contain additional events which are referenced by event instruments but were not added to the bank, and those referenced events are not counted.
    pub fn get_event_list(&self) -> Result<Vec<EventDescription>> {
        let expected_count = self.event_count()?;
        let mut count = 0;
        let mut list = vec![std::ptr::null_mut(); expected_count as usize];

        unsafe {
            FMOD_Studio_Bank_GetEventList(
                self.inner.as_ptr(),
                // bus is repr transparent and has the same layout as *mut FMOD_STUDIO_BUS, so this cast is ok
                list.as_mut_ptr(),
                list.capacity() as c_int,
                &raw mut count,
            )
            .to_result()?;

            debug_assert_eq!(count, expected_count);

            Ok(std::mem::transmute::<
                Vec<*mut fmod_sys::FMOD_STUDIO_EVENTDESCRIPTION>,
                Vec<EventDescription>,
            >(list))
        }
    }

    /// Retrieves the number of string table entries in the bank.
    pub fn string_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_Bank_GetStringCount(self.inner.as_ptr(), &raw mut count).to_result()?;
        }
        Ok(count)
    }

    /// Retrieves a string table entry.
    ///
    /// May be used in conjunction with [`Bank::string_count`] to enumerate the string table in a bank.
    pub fn get_string_info(&self, index: c_int) -> Result<(Guid, Utf8CString)> {
        let mut guid = MaybeUninit::zeroed();
        let path = get_string_out_size(|path, size, ret| unsafe {
            FMOD_Studio_Bank_GetStringInfo(
                self.inner.as_ptr(),
                index,
                guid.as_mut_ptr(),
                path,
                size,
                ret,
            )
        })?;
        let guid = unsafe { guid.assume_init().into() };
        Ok((guid, path))
    }

    /// Retrieves the number of VCAs in the bank.
    pub fn vca_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_Bank_GetVCACount(self.inner.as_ptr(), &raw mut count).to_result()?;
        }
        Ok(count)
    }

    /// Retrieves a list of the VCAs in the bank.
    pub fn get_vca_list(&self) -> Result<Vec<Vca>> {
        let expected_count = self.event_count()?;
        let mut count = 0;
        let mut list = vec![
            Vca {
                inner: NonNull::dangling(), // same trick used here as getting the bus list
            };
            expected_count as usize
        ];

        unsafe {
            FMOD_Studio_Bank_GetVCAList(
                self.inner.as_ptr(),
                // bus is repr transparent and has the same layout as *mut FMOD_STUDIO_BUS, so this cast is ok
                list.as_mut_ptr().cast::<*mut FMOD_STUDIO_VCA>(),
                list.capacity() as c_int,
                &raw mut count,
            )
            .to_result()?;

            debug_assert_eq!(count, expected_count);

            Ok(list)
        }
    }
}
