// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use fmod_sys::*;
use lanyard::{Utf8CStr, Utf8CString};
use std::{
    ffi::{c_float, c_int},
    mem::MaybeUninit,
};

use crate::studio::{ParameterDescription, ParameterID, System, get_string_out_size};
use crate::{FmodResultExt, Result};

impl System {
    /// Retrieves a global parameter value by unique identifier.
    ///
    /// The second tuple field is the final value of the parameter after applying adjustments due to automation, modulation, seek speed, and parameter velocity to value.
    /// This is calculated asynchronously when the Studio system updates.
    pub fn get_parameter_by_id(&self, id: ParameterID) -> Result<(c_float, c_float)> {
        let mut value = 0.0;
        let mut final_value = 0.0;

        unsafe {
            FMOD_Studio_System_GetParameterByID(
                self.as_ptr(),
                id.into(),
                &raw mut value,
                &raw mut final_value,
            )
            .to_result()?;
        }

        Ok((value, final_value))
    }

    /// Sets a global parameter value by unique identifier.
    pub fn set_parameter_by_id(
        &self,
        id: ParameterID,
        value: c_float,
        ignore_seek_speed: bool,
    ) -> Result<()> {
        unsafe {
            FMOD_Studio_System_SetParameterByID(
                self.as_ptr(),
                id.into(),
                value,
                ignore_seek_speed.into(),
            )
            .to_result()
        }
    }

    /// Sets a global parameter value by unique identifier, looking up the value label.
    ///
    /// If the specified label is not found, [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned.
    /// This lookup is case sensitive.
    pub fn set_parameter_by_id_with_label(
        &self,
        id: ParameterID,
        label: &Utf8CStr,
        ignore_seek_speed: bool,
    ) -> Result<()> {
        unsafe {
            FMOD_Studio_System_SetParameterByIDWithLabel(
                self.as_ptr(),
                id.into(),
                label.as_ptr(),
                ignore_seek_speed.into(),
            )
            .to_result()
        }
    }

    /// Sets multiple global parameter values by unique identifier.
    ///
    /// If any ID is set to all zeroes then the corresponding value will be ignored.
    ///
    /// # Panics
    ///
    /// This function will panic if `ids.len()` != `values.len()`.
    pub fn set_parameters_by_ids(
        &self,
        ids: &[ParameterID], // TODO fmod says that the size of this must range from 1-32. do we need to enforce this?
        values: &mut [c_float], // TODO is this &mut correct? does fmod perform any writes?
        ignore_seek_speed: bool,
    ) -> Result<()> {
        // TODO don't panic, return result
        assert_eq!(ids.len(), values.len());

        unsafe {
            FMOD_Studio_System_SetParametersByIDs(
                self.as_ptr(),
                ids.as_ptr().cast(),
                values.as_mut_ptr(),
                ids.len() as c_int,
                ignore_seek_speed.into(),
            )
            .to_result()
        }
    }

    /// Retrieves a global parameter value by name.
    ///
    /// The second tuple field is the final value of the parameter after applying adjustments due to automation, modulation, seek speed, and parameter velocity to value.
    /// This is calculated asynchronously when the Studio system updates.
    pub fn get_parameter_by_name(&self, name: &Utf8CStr) -> Result<(c_float, c_float)> {
        let mut value = 0.0;
        let mut final_value = 0.0;

        unsafe {
            FMOD_Studio_System_GetParameterByName(
                self.as_ptr(),
                name.as_ptr(),
                &raw mut value,
                &raw mut final_value,
            )
            .to_result()?;
        }

        Ok((value, final_value))
    }

    /// Sets a global parameter value by name.
    pub fn set_parameter_by_name(
        &self,
        name: &Utf8CStr,
        value: c_float,
        ignore_seek_speed: bool,
    ) -> Result<()> {
        unsafe {
            FMOD_Studio_System_SetParameterByName(
                self.as_ptr(),
                name.as_ptr(),
                value,
                ignore_seek_speed.into(),
            )
            .to_result()
        }
    }

    /// Sets a global parameter value by name, looking up the value label.
    ///
    /// If the specified label is not found, [`FMOD_RESULT::FMOD_ERR_EVENT_NOTFOUND`] is returned. This lookup is case sensitive.
    pub fn set_parameter_by_name_with_label(
        &self,
        name: &Utf8CStr,
        label: &Utf8CStr,
        ignore_seek_speed: bool,
    ) -> Result<()> {
        unsafe {
            FMOD_Studio_System_SetParameterByNameWithLabel(
                self.as_ptr(),
                name.as_ptr(),
                label.as_ptr(),
                ignore_seek_speed.into(),
            )
            .to_result()
        }
    }

    /// Retrieves a global parameter by name or path.
    ///
    /// `name` can be the short name (such as `Wind`) or the full path (such as `parameter:/Ambience/Wind`).
    /// Path lookups will only succeed if the strings bank has been loaded.
    pub fn get_parameter_description_by_name(
        &self,
        name: &Utf8CStr,
    ) -> Result<ParameterDescription> {
        let mut description = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_System_GetParameterDescriptionByName(
                self.as_ptr(),
                name.as_ptr(),
                description.as_mut_ptr(),
            )
            .to_result()?;

            let description = ParameterDescription::from_ffi(description.assume_init());
            Ok(description)
        }
    }

    /// Retrieves a global parameter by ID.
    pub fn get_parameter_description_by_id(&self, id: ParameterID) -> Result<ParameterDescription> {
        let mut description = MaybeUninit::zeroed();
        unsafe {
            FMOD_Studio_System_GetParameterDescriptionByID(
                self.as_ptr(),
                id.into(),
                description.as_mut_ptr(),
            )
            .to_result()?;

            let description = ParameterDescription::from_ffi(description.assume_init());
            Ok(description)
        }
    }

    /// Retrieves the number of global parameters.
    pub fn parameter_description_count(&self) -> Result<c_int> {
        let mut count = 0;
        unsafe {
            FMOD_Studio_System_GetParameterDescriptionCount(self.as_ptr(), &raw mut count)
                .to_result()?;
        }
        Ok(count)
    }

    /// Retrieves a list of global parameters.
    pub fn get_parameter_description_list(&self) -> Result<Vec<ParameterDescription>> {
        let expected_count = self.parameter_description_count()?;
        let mut count = 0;
        let mut list = vec![MaybeUninit::zeroed(); expected_count as usize];

        unsafe {
            FMOD_Studio_System_GetParameterDescriptionList(
                self.as_ptr(),
                // bank is repr transparent and has the same layout as *mut FMOD_STUDIO_BANK, so this cast is ok
                list.as_mut_ptr()
                    .cast::<FMOD_STUDIO_PARAMETER_DESCRIPTION>(),
                list.capacity() as c_int,
                &raw mut count,
            )
            .to_result()?;

            debug_assert_eq!(count, expected_count);

            let list = list
                .into_iter()
                .map(|uninit| {
                    let description = uninit.assume_init();
                    ParameterDescription::from_ffi(description)
                })
                .collect();

            Ok(list)
        }
    }

    /// Retrieves a global parameter label by name or path.
    ///
    /// `name` can be the short name (such as `Wind`) or the full path (such as `parameter:/Ambience/Wind`).
    /// Path lookups will only succeed if the strings bank has been loaded.
    pub fn get_parameter_label_by_name(
        &self,
        name: &Utf8CStr,
        label_index: c_int,
    ) -> Result<Utf8CString> {
        get_string_out_size(|path, size, ret| unsafe {
            FMOD_Studio_System_GetParameterLabelByName(
                self.as_ptr(),
                name.as_ptr(),
                label_index,
                path,
                size,
                ret,
            )
        })
    }

    /// Retrieves a global parameter label by ID.
    pub fn get_parameter_label_by_id(
        &self,
        id: ParameterID,
        label_index: c_int,
    ) -> Result<Utf8CString> {
        get_string_out_size(|path, size, ret| unsafe {
            FMOD_Studio_System_GetParameterLabelByID(
                self.as_ptr(),
                id.into(),
                label_index,
                path,
                size,
                ret,
            )
        })
    }
}
