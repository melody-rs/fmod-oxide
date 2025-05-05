// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::Utf8CStr;
use std::ffi::{c_char, c_int, c_void};
use std::marker::PhantomData;

use crate::studio::{Bank, LoadBankFlags, System};
use crate::{
    FileSystemSync, Guid, filesystem_close, filesystem_open, filesystem_read, filesystem_seek,
};

#[cfg(doc)]
use crate::studio::AdvancedSettings;

/// User data to be passed to the file callbacks.
#[derive(Debug, Clone, Copy)]
pub struct LoadBankUserdata<'a> {
    data: *mut c_void,
    size: c_int,
    _marker: PhantomData<&'a [u8]>,
}

impl Default for LoadBankUserdata<'_> {
    fn default() -> Self {
        Self {
            data: std::ptr::null_mut(),
            size: 0,
            _marker: PhantomData,
        }
    }
}

impl<'a> LoadBankUserdata<'a> {
    /// userdata will be copied internally;
    /// this copy will be kept until the bank has been unloaded and all calls to
    /// [`FileSystem::open`] have been matched by a call to [`FileSystem::close`].
    pub fn from_slice(slice: &'a [u8]) -> Self {
        Self {
            data: slice.as_ptr().cast::<c_void>().cast_mut(),
            size: slice.len() as c_int,
            _marker: PhantomData,
        }
    }

    /// # Safety
    ///
    /// `userdata` must remain valid until the bank has been unloadedand all calls to
    /// [`FileSystem::open`] have been matched by a call to [`FileSystem::close`].
    ///
    /// This requirement allows playback of shared streaming assets to continue after a bank is unloaded.
    pub unsafe fn from_pointer(userdata: *mut c_void) -> Self {
        Self {
            data: userdata,
            size: 0,
            _marker: PhantomData,
        }
    }
}

impl System {
    /// Loads the metadata of a bank using custom read callbacks.
    ///
    /// Sample data must be loaded separately see Sample Data Loading for details.
    ///
    /// By default this function blocks until the load finishes.
    ///
    /// Using the [`LoadBankFlags::NONBLOCKING`] flag causes the bank to be loaded asynchronously.
    /// In that case, this function always returns [`Ok`] and bank contains a valid bank handle.
    /// Load errors for asynchronous banks can be detected by calling [`Bank::get_loading_state`].
    /// Failed asynchronous banks should be released by calling [`Bank::unload`].
    ///
    /// If a bank is split, separating out sample data and optionally streams from the metadata bank,
    /// all parts must be loaded before any APIs that use the data are called.
    /// We recommend you load each part one after another (the order in which they are loaded is not important),
    /// then proceed with dependent API calls such as [`Bank::load_sample_data`] or [`System::get_event`].
    pub fn load_bank_custom<F: FileSystemSync>(
        &self,
        userdata: LoadBankUserdata<'_>,
        load_flags: LoadBankFlags,
    ) -> Result<Bank> {
        let bank_info = FMOD_STUDIO_BANK_INFO {
            size: size_of::<FMOD_STUDIO_BANK_INFO>() as c_int,
            userdata: userdata.data,
            userdatalength: userdata.size,
            opencallback: Some(filesystem_open::<F>),
            closecallback: Some(filesystem_close::<F>),
            readcallback: Some(filesystem_read::<F>),
            seekcallback: Some(filesystem_seek::<F>),
        };

        let mut bank = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_LoadBankCustom(
                self.inner.as_ptr(),
                &raw const bank_info,
                load_flags.into(),
                &raw mut bank,
            )
            .to_result()?;

            Ok(Bank::from_ffi(bank))
        }
    }

    /// Sample data must be loaded separately.
    ///
    /// By default this function will block until the file load finishes.
    ///
    /// Using the [`LoadBankFlags::NONBLOCKING`] flag will cause the bank to be loaded asynchronously.
    /// In that case this function will always return [`Ok`] and bank will contain a valid bank handle.
    /// Load errors for asynchronous banks can be detected by calling [`Bank::get_loading_state`].
    /// Failed asynchronous banks should be released by calling [`Bank::unload`].
    ///
    /// If a bank has been split, separating out assets and optionally streams from the metadata bank, all parts must be loaded before any APIs that use the data are called.
    /// It is recommended you load each part one after another (order is not important), then proceed with dependent API calls such as [`Bank::load_sample_data`] or [`System::get_event`].
    pub fn load_bank_file(&self, filename: &Utf8CStr, load_flags: LoadBankFlags) -> Result<Bank> {
        let mut bank = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_LoadBankFile(
                self.inner.as_ptr(),
                filename.as_ptr(),
                load_flags.bits(),
                &raw mut bank,
            )
            .to_result()?;
            Ok(Bank::from_ffi(bank))
        }
    }

    /// Sample data must be loaded separately.
    ///
    /// This function is the safe counterpart of [`System::load_bank_pointer`].
    /// FMOD will allocate an internal buffer and copy the data from the passed in buffer before using it.
    /// The buffer passed to this function may be cleaned up at any time after this function returns.
    ///
    /// By default this function will block until the load finishes.
    ///
    /// Using the [`LoadBankFlags::NONBLOCKING`] flag will cause the bank to be loaded asynchronously.
    /// In that case this function will always return [`Ok`] and bank will contain a valid bank handle.
    /// Load errors for asynchronous banks can be detected by calling [`Bank::get_loading_state`].
    /// Failed asynchronous banks should be released by calling [`Bank::unload`].
    ///
    /// This function is not compatible with [`AdvancedSettings::encryption_key`], using them together will cause an error to be returned.
    ///
    /// If a bank has been split, separating out assets and optionally streams from the metadata bank, all parts must be loaded before any APIs that use the data are called.
    /// It is recommended you load each part one after another (order is not important), then proceed with dependent API calls such as [`Bank::load_sample_data`] or [`System::get_event`].
    pub fn load_bank_memory(&self, buffer: &[u8], flags: LoadBankFlags) -> Result<Bank> {
        let mut bank = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_LoadBankMemory(
                self.inner.as_ptr(),
                buffer.as_ptr().cast::<c_char>(),
                buffer.len() as c_int,
                FMOD_STUDIO_LOAD_MEMORY,
                flags.bits(),
                &raw mut bank,
            )
            .to_result()?;
            Ok(Bank::from_ffi(bank))
        }
    }

    /// Sample data must be loaded separately.
    ///
    /// This function is the unsafe counterpart of [`System::load_bank_memory`].
    /// FMOD will use the passed memory buffer directly.
    ///
    /// By default this function will block until the load finishes.
    ///
    /// Using the [`LoadBankFlags::NONBLOCKING`] flag will cause the bank to be loaded asynchronously.
    /// In that case this function will always return [`Ok`] and bank will contain a valid bank handle.
    /// Load errors for asynchronous banks can be detected by calling [`Bank::get_loading_state`].
    /// Failed asynchronous banks should be released by calling [`Bank::unload`].
    ///
    /// This function is not compatible with [`AdvancedSettings::encryption_key`], using them together will cause an error to be returned.
    ///
    /// If a bank has been split, separating out assets and optionally streams from the metadata bank, all parts must be loaded before any APIs that use the data are called.
    /// It is recommended you load each part one after another (order is not important), then proceed with dependent API calls such as [`Bank::load_sample_data`] or [`System::get_event`].
    ///
    /// # Safety
    /// When using this function the buffer must be aligned to [`FMOD_STUDIO_LOAD_MEMORY_ALIGNMENT`]
    /// and the memory must persist until the bank has been fully unloaded, which can be some time after calling [`Bank::unload`] to unload the bank.
    /// You can ensure the memory is not being freed prematurely by only freeing it after receiving the [`FMOD_STUDIO_SYSTEM_CALLBACK_BANK_UNLOAD`] callback.
    pub unsafe fn load_bank_pointer(&self, buffer: &[u8], flags: LoadBankFlags) -> Result<Bank> {
        let mut bank = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_LoadBankMemory(
                self.inner.as_ptr(),
                buffer.as_ptr().cast::<c_char>(),
                buffer.len() as c_int,
                FMOD_STUDIO_LOAD_MEMORY_POINT,
                flags.bits(),
                &raw mut bank,
            )
            .to_result()?;
            Ok(Bank::from_ffi(bank))
        }
    }

    /// Unloads all currently loaded banks.
    pub fn unload_all_banks(&self) -> Result<()> {
        unsafe { FMOD_Studio_System_UnloadAll(self.inner.as_ptr()).to_result() }
    }

    /// Retrieves a loaded bank
    ///
    /// `path_or_id` may be a path, such as `bank:/Weapons` or an ID string such as `{793cddb6-7fa1-4e06-b805-4c74c0fd625b}`.
    ///
    /// Note that path lookups will only succeed if the strings bank has been loaded.
    pub fn get_bank(&self, path_or_id: &Utf8CStr) -> Result<Bank> {
        let mut bank = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetBank(self.inner.as_ptr(), path_or_id.as_ptr(), &raw mut bank)
                .to_result()?;
            Ok(Bank::from_ffi(bank))
        }
    }

    /// Retrieves a loaded bank.
    pub fn get_bank_by_id(&self, id: Guid) -> Result<Bank> {
        let mut bank = std::ptr::null_mut();
        unsafe {
            FMOD_Studio_System_GetBankByID(self.inner.as_ptr(), &id.into(), &raw mut bank)
                .to_result()?;
            Ok(Bank::from_ffi(bank))
        }
    }

    /// Retrieves the number of loaded banks.
    pub fn bank_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_System_GetBankCount(self.inner.as_ptr(), &raw mut count).to_result()?;
        }
        Ok(count)
    }

    /// Retrieves a list of the currently-loaded banks.
    pub fn get_bank_list(&self) -> Result<Vec<Bank>> {
        let expected_count = self.bank_count()?;
        let mut count = 0;
        let mut list = vec![std::ptr::null_mut(); expected_count as usize];

        unsafe {
            FMOD_Studio_System_GetBankList(
                self.inner.as_ptr(),
                // bank is repr transparent and has the same layout as *mut FMOD_STUDIO_BANK, so this cast is ok
                list.as_mut_ptr(),
                list.capacity() as c_int,
                &raw mut count,
            )
            .to_result()?;

            debug_assert_eq!(count, expected_count);

            Ok(std::mem::transmute::<
                Vec<*mut fmod_sys::FMOD_STUDIO_BANK>,
                Vec<Bank>,
            >(list))
        }
    }
}
