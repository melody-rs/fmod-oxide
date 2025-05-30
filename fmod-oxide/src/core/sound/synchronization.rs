// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    ffi::{c_int, c_uint},
    ptr::NonNull,
};

use crate::{FmodResultExt, Result};
use fmod_sys::*;
use lanyard::{Utf8CStr, Utf8CString};

use crate::{Sound, TimeUnit, get_string};

/// Named marker for a given point in time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)] // so we can transmute between types
pub struct SyncPoint {
    pub(crate) inner: NonNull<FMOD_SYNCPOINT>,
}

#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Send for SyncPoint {}
#[cfg(not(feature = "thread-unsafe"))]
unsafe impl Sync for SyncPoint {}

impl SyncPoint {
    /// # Safety
    ///
    /// `value` must be a valid pointer either aquired from [`Self::as_ptr`] or FMOD.
    ///
    /// # Panics
    ///
    /// Panics if `value` is null.
    pub unsafe fn from_ffi(value: *mut FMOD_SYNCPOINT) -> Self {
        let inner = NonNull::new(value).unwrap();
        SyncPoint { inner }
    }

    /// Converts `self` into its raw representation.
    pub fn as_ptr(self) -> *mut FMOD_SYNCPOINT {
        self.inner.as_ptr()
    }
}

impl From<SyncPoint> for *mut FMOD_SYNCPOINT {
    fn from(value: SyncPoint) -> Self {
        value.inner.as_ptr()
    }
}

impl Sound {
    /// Retrieve a sync point.
    ///
    /// For for more information on sync points see Sync Points.
    pub fn get_sync_point(&self, index: i32) -> Result<SyncPoint> {
        let mut sync_point = std::ptr::null_mut();
        unsafe {
            FMOD_Sound_GetSyncPoint(self.inner.as_ptr(), index, &raw mut sync_point).to_result()?;
            Ok(SyncPoint::from_ffi(sync_point))
        }
    }

    /// Retrieves information on an embedded sync point.
    ///
    /// For for more information on sync points see Sync Points.
    pub fn get_sync_point_info(
        &self,
        point: SyncPoint,
        offset_type: TimeUnit,
    ) -> Result<(Utf8CString, c_uint)> {
        let mut offset = 0;
        let name = get_string(|name| unsafe {
            FMOD_Sound_GetSyncPointInfo(
                self.inner.as_ptr(),
                point.into(),
                name.as_mut_ptr().cast(),
                name.len() as c_int,
                &raw mut offset,
                offset_type.into(),
            )
        })?;
        Ok((name, offset))
    }

    /// Retrieves the number of sync points stored within a sound.
    ///
    /// For for more information on sync points see Sync Points.
    pub fn get_sync_point_count(&self) -> Result<i32> {
        let mut count = 0;
        unsafe {
            FMOD_Sound_GetNumSyncPoints(self.inner.as_ptr(), &raw mut count).to_result()?;
        }
        Ok(count)
    }

    /// Adds a sync point at a specific time within the sound.
    ///
    /// For more information on sync points see Sync Points.
    pub fn add_sync_point(
        &self,
        offset: c_uint,
        offset_type: TimeUnit,
        name: &Utf8CStr,
    ) -> Result<SyncPoint> {
        let mut sync_point = std::ptr::null_mut();
        unsafe {
            FMOD_Sound_AddSyncPoint(
                self.inner.as_ptr(),
                offset,
                offset_type.into(),
                name.as_ptr(),
                &raw mut sync_point,
            )
            .to_result()?;
            Ok(SyncPoint::from_ffi(sync_point))
        }
    }

    /// Deletes a sync point within the sound.
    ///
    /// For for more information on sync points see Sync Points.
    pub fn delete_sync_point(&self, point: SyncPoint) -> Result<()> {
        unsafe {
            FMOD_Sound_DeleteSyncPoint(self.inner.as_ptr(), point.into()).to_result()?;
        }
        Ok(())
    }
}
