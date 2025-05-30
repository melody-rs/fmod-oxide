// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::ReverbProperties;

/// Predefined reverb configurations. To simplify usage, and avoid manually selecting reverb parameters, a table of common presets is supplied for ease of use.
// bindgen doesn't generate these so we have to do this ourselves.
// hang on, these look like copies from OpenAL EAX's Reverb presets?
impl ReverbProperties {
    /// off/disabled.
    pub const OFF: Self = Self {
        decay_time: 1000.0,
        early_delay: 7.0,
        late_delay: 11.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 100.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 20.0,
        early_late_mix: 96.0,
        wet_level: -80.0,
    };
    /// Generic/default
    pub const GENERIC: Self = Self {
        decay_time: 1500.0,
        early_delay: 7.0,
        late_delay: 11.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 83.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 14500.0,
        early_late_mix: 96.0,
        wet_level: -8.0,
    };
    /// Padded cell
    pub const PADDEDCELL: Self = Self {
        decay_time: 170.0,
        early_delay: 1.0,
        late_delay: 2.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 10.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 160.0,
        early_late_mix: 84.0,
        wet_level: -7.8,
    };
    /// Room
    pub const ROOM: Self = Self {
        decay_time: 400.0,
        early_delay: 2.0,
        late_delay: 3.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 83.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 6050.0,
        early_late_mix: 88.0,
        wet_level: -9.4,
    };
    /// Bathroom
    pub const BATHROOM: Self = Self {
        decay_time: 1500.0,
        early_delay: 7.0,
        late_delay: 11.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 54.0,
        diffusion: 100.0,
        density: 60.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 2900.0,
        early_late_mix: 83.0,
        wet_level: 0.5,
    };
    /// Living room
    pub const LIVINGROOM: Self = Self {
        decay_time: 500.0,
        early_delay: 3.0,
        late_delay: 4.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 10.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 160.0,
        early_late_mix: 58.0,
        wet_level: -19.0,
    };
    /// Stone room
    pub const STONEROOM: Self = Self {
        decay_time: 2300.0,
        early_delay: 12.0,
        late_delay: 17.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 64.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 7800.0,
        early_late_mix: 71.0,
        wet_level: -8.5,
    };
    /// Auditorium
    pub const AUDITORIUM: Self = Self {
        decay_time: 4300.0,
        early_delay: 20.0,
        late_delay: 30.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 59.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 5850.0,
        early_late_mix: 64.0,
        wet_level: -11.7,
    };
    /// Concert hall
    pub const CONCERTHALL: Self = Self {
        decay_time: 3900.0,
        early_delay: 20.0,
        late_delay: 29.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 70.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 5650.0,
        early_late_mix: 80.0,
        wet_level: -9.8,
    };
    /// Cave
    pub const CAVE: Self = Self {
        decay_time: 2900.0,
        early_delay: 15.0,
        late_delay: 22.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 100.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 20000.0,
        early_late_mix: 59.0,
        wet_level: -11.3,
    };
    /// Arena
    pub const ARENA: Self = Self {
        decay_time: 7200.0,
        early_delay: 20.0,
        late_delay: 30.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 33.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 4500.0,
        early_late_mix: 80.0,
        wet_level: -9.6,
    };
    /// Hangar
    pub const HANGAR: Self = Self {
        decay_time: 10000.0,
        early_delay: 20.0,
        late_delay: 30.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 23.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 3400.0,
        early_late_mix: 72.0,
        wet_level: -7.4,
    };
    /// Carpetted hallway
    pub const CARPETTEDHALLWAY: Self = Self {
        decay_time: 300.0,
        early_delay: 2.0,
        late_delay: 30.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 10.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 500.0,
        early_late_mix: 56.0,
        wet_level: -24.0,
    };
    /// Hallway
    pub const HALLWAY: Self = Self {
        decay_time: 1500.0,
        early_delay: 7.0,
        late_delay: 11.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 59.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 7800.0,
        early_late_mix: 87.0,
        wet_level: -5.5,
    };
    /// Stone corridor
    pub const STONECORRIDOR: Self = Self {
        decay_time: 270.0,
        early_delay: 13.0,
        late_delay: 20.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 79.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 9000.0,
        early_late_mix: 86.0,
        wet_level: -6.0,
    };
    /// Alley
    pub const ALLEY: Self = Self {
        decay_time: 1500.0,
        early_delay: 7.0,
        late_delay: 11.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 86.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 8300.0,
        early_late_mix: 80.0,
        wet_level: -9.8,
    };
    /// Forest
    pub const FOREST: Self = Self {
        decay_time: 1500.0,
        early_delay: 162.0,
        late_delay: 88.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 54.0,
        diffusion: 79.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 760.0,
        early_late_mix: 94.0,
        wet_level: -12.3,
    };
    /// City
    pub const CITY: Self = Self {
        decay_time: 1500.0,
        early_delay: 7.0,
        late_delay: 11.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 67.0,
        diffusion: 50.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 4050.0,
        early_late_mix: 66.0,
        wet_level: -26.0,
    };
    /// Mountains
    pub const MOUNTAINS: Self = Self {
        decay_time: 1500.0,
        early_delay: 300.0,
        late_delay: 100.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 21.0,
        diffusion: 27.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 1220.0,
        early_late_mix: 82.0,
        wet_level: -24.0,
    };
    /// Quarry
    pub const QUARRY: Self = Self {
        decay_time: 1500.0,
        early_delay: 61.0,
        late_delay: 25.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 83.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 3400.0,
        early_late_mix: 100.0,
        wet_level: -5.0,
    };
    /// Plain
    pub const PLAIN: Self = Self {
        decay_time: 1500.0,
        early_delay: 179.0,
        late_delay: 100.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 50.0,
        diffusion: 21.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 1670.0,
        early_late_mix: 65.0,
        wet_level: -28.0,
    };
    /// Parking lot
    pub const PARKINGLOT: Self = Self {
        decay_time: 1700.0,
        early_delay: 8.0,
        late_delay: 12.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 100.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 20000.0,
        early_late_mix: 56.0,
        wet_level: -19.5,
    };
    /// Sewer pipe
    pub const SEWERPIPE: Self = Self {
        decay_time: 2800.0,
        early_delay: 14.0,
        late_delay: 21.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 14.0,
        diffusion: 80.0,
        density: 60.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 3400.0,
        early_late_mix: 66.0,
        wet_level: 1.2,
    };
    /// Underwater
    pub const UNDERWATER: Self = Self {
        decay_time: 1500.0,
        early_delay: 7.0,
        late_delay: 11.0,
        hf_reference: 5000.0,
        hf_decay_ratio: 10.0,
        diffusion: 100.0,
        density: 100.0,
        low_shelf_frequency: 250.0,
        low_shelf_gain: 0.0,
        high_cut: 500.0,
        early_late_mix: 92.0,
        wet_level: 7.0,
    };
}
