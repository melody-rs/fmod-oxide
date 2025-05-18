// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use indexmap::{IndexMap, IndexSet};

pub struct CInfo {
    pub categories: IndexSet<String>,
    pub functions: IndexMap<String, CFunction>,
    pub enums: IndexMap<String, CEnum>,
    pub macros: IndexMap<String, bool>,
    pub structs: IndexMap<String, bool>,
}

pub struct CFunction {
    pub category: usize,
    pub marked: bool,
}

pub struct CEnum {
    pub variants: IndexMap<String, bool>,
}

const FILTER: &[&str] = &[
    // defined in the FMOD header but never used
    "FMOD_POLYGON",
];

const PRE_MARKED: &[&str] = &[
    // these are covered but not referred to directly.
    // our simple coverage tools cant pick them up
    "FMOD_DSP_LOUDNESS_METER_INFO_TYPE",
    "FMOD_DSP_LOUDNESS_METER_WEIGHTING_TYPE",
    "FMOD_DSP_PARAMETER_FLOAT_MAPPING_PIECEWISE_LINEAR",
    "FMOD_DSP_PARAMETER_FLOAT_MAPPING",
    "FMOD_DSP_PARAMETER_DESC_FLOAT",
    "FMOD_DSP_PARAMETER_DESC_INT",
    "FMOD_DSP_PARAMETER_DESC_BOOL",
    "FMOD_DSP_PARAMETER_DESC_DATA",
    "FMOD_DSP_PARAMETER_OVERALLGAIN",
    "FMOD_DSP_PARAMETER_3DATTRIBUTES",
    "FMOD_DSP_PARAMETER_3DATTRIBUTES_MULTI",
    "FMOD_DSP_PARAMETER_ATTENUATION_RANGE",
    "FMOD_DSP_PARAMETER_DYNAMIC_RESPONSE",
    // covered but use const generics
    "FMOD_DSP_DELAY_CH1",
    "FMOD_DSP_DELAY_CH2",
    "FMOD_DSP_DELAY_CH3",
    "FMOD_DSP_DELAY_CH4",
    "FMOD_DSP_DELAY_CH5",
    "FMOD_DSP_DELAY_CH6",
    "FMOD_DSP_DELAY_CH7",
    "FMOD_DSP_DELAY_CH8",
    "FMOD_DSP_DELAY_CH9",
    "FMOD_DSP_DELAY_CH10",
    "FMOD_DSP_DELAY_CH11",
    "FMOD_DSP_DELAY_CH12",
    "FMOD_DSP_DELAY_CH13",
    "FMOD_DSP_DELAY_CH14",
    "FMOD_DSP_DELAY_CH15",
    // ditto
    "FMOD_DSP_CHANNELMIX_GAIN_CH1",
    "FMOD_DSP_CHANNELMIX_GAIN_CH2",
    "FMOD_DSP_CHANNELMIX_GAIN_CH3",
    "FMOD_DSP_CHANNELMIX_GAIN_CH4",
    "FMOD_DSP_CHANNELMIX_GAIN_CH5",
    "FMOD_DSP_CHANNELMIX_GAIN_CH6",
    "FMOD_DSP_CHANNELMIX_GAIN_CH7",
    "FMOD_DSP_CHANNELMIX_GAIN_CH8",
    "FMOD_DSP_CHANNELMIX_GAIN_CH9",
    "FMOD_DSP_CHANNELMIX_GAIN_CH10",
    "FMOD_DSP_CHANNELMIX_GAIN_CH11",
    "FMOD_DSP_CHANNELMIX_GAIN_CH12",
    "FMOD_DSP_CHANNELMIX_GAIN_CH13",
    "FMOD_DSP_CHANNELMIX_GAIN_CH14",
    "FMOD_DSP_CHANNELMIX_GAIN_CH15",
    "FMOD_DSP_CHANNELMIX_GAIN_CH16",
    "FMOD_DSP_CHANNELMIX_GAIN_CH17",
    "FMOD_DSP_CHANNELMIX_GAIN_CH18",
    "FMOD_DSP_CHANNELMIX_GAIN_CH19",
    "FMOD_DSP_CHANNELMIX_GAIN_CH20",
    "FMOD_DSP_CHANNELMIX_GAIN_CH21",
    "FMOD_DSP_CHANNELMIX_GAIN_CH22",
    "FMOD_DSP_CHANNELMIX_GAIN_CH23",
    "FMOD_DSP_CHANNELMIX_GAIN_CH24",
    "FMOD_DSP_CHANNELMIX_GAIN_CH25",
    "FMOD_DSP_CHANNELMIX_GAIN_CH26",
    "FMOD_DSP_CHANNELMIX_GAIN_CH27",
    "FMOD_DSP_CHANNELMIX_GAIN_CH28",
    "FMOD_DSP_CHANNELMIX_GAIN_CH29",
    "FMOD_DSP_CHANNELMIX_GAIN_CH30",
    "FMOD_DSP_CHANNELMIX_GAIN_CH31",
    // same as above
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH1",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH2",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH3",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH4",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH5",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH6",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH7",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH8",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH9",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH10",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH11",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH12",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH13",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH14",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH15",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH16",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH17",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH18",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH19",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH20",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH21",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH22",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH23",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH24",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH25",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH26",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH27",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH28",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH29",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH30",
    "FMOD_DSP_CHANNELMIX_OUTPUT_CH31",
    // these are covered but not reffered to directly
    "FMOD_PRESET_OFF",
    "FMOD_PRESET_GENERIC",
    "FMOD_PRESET_PADDEDCELL",
    "FMOD_PRESET_ROOM",
    "FMOD_PRESET_BATHROOM",
    "FMOD_PRESET_LIVINGROOM",
    "FMOD_PRESET_STONEROOM",
    "FMOD_PRESET_AUDITORIUM",
    "FMOD_PRESET_CONCERTHALL",
    "FMOD_PRESET_CAVE",
    "FMOD_PRESET_ARENA",
    "FMOD_PRESET_HANGAR",
    "FMOD_PRESET_CARPETTEDHALLWAY",
    "FMOD_PRESET_HALLWAY",
    "FMOD_PRESET_STONECORRIDOR",
    "FMOD_PRESET_ALLEY",
    "FMOD_PRESET_FOREST",
    "FMOD_PRESET_CITY",
    "FMOD_PRESET_MOUNTAINS",
    "FMOD_PRESET_QUARRY",
    "FMOD_PRESET_PLAIN",
    "FMOD_PRESET_PARKINGLOT",
    "FMOD_PRESET_SEWERPIPE",
    "FMOD_PRESET_UNDERWATER",
];

pub fn collect(
    translation_unit: &clang::TranslationUnit<'_>,
    verbose: bool,
) -> color_eyre::Result<CInfo> {
    let entities = translation_unit.get_entity().get_children();

    let category_regex = regex::Regex::new(r"FMOD_(Studio_)?([A-Za-z0-9]*)_.*$")?;
    // all fmod macros start with FMOD_
    let macro_regex = regex::Regex::new(r"^FMOD_.*$")?;

    let mut categories = IndexSet::new();
    let mut functions = IndexMap::new();
    let mut macros = IndexMap::new();
    let mut enums = IndexMap::new();
    let mut structs = IndexMap::new();

    // setup seen with an initial filter for functions we don't want to include (for any particular reason)
    let mut seen: std::collections::HashSet<_> = FILTER.iter().copied().map(String::from).collect();
    let pre_marked: std::collections::HashSet<_> = PRE_MARKED.iter().copied().collect();

    for entity in entities {
        if let Some(name) = entity.get_name() {
            if !seen.insert(name) {
                continue;
            }
        }

        match entity.get_kind() {
            clang::EntityKind::MacroDefinition => {
                let name = entity.get_name().unwrap();
                if macro_regex.is_match(&name) {
                    if verbose {
                        println!("Found Macro: {name}");
                    }

                    let pre_marked = pre_marked.contains(name.as_str());
                    macros.insert(name, pre_marked);
                }
            }
            clang::EntityKind::EnumDecl => {
                let name = entity.get_name().unwrap();
                let variants = entity
                    .get_children()
                    .iter()
                    .filter_map(|variant| {
                        let name = variant.get_name().unwrap();
                        if name.ends_with("_MAX") || name.ends_with("_FORCEINT") {
                            None
                        } else {
                            let pre_marked = pre_marked.contains(name.as_str());
                            Some((name, pre_marked))
                        }
                    })
                    .collect();
                enums.insert(name, CEnum { variants });
            }
            clang::EntityKind::StructDecl => {
                let name = entity.get_name().unwrap();
                let pre_marked = pre_marked.contains(name.as_str());
                structs.insert(name, pre_marked);
            }
            clang::EntityKind::FunctionDecl => {
                let name = entity.get_name().unwrap();
                let category = category_regex
                    .captures(&name)
                    .map(|c| {
                        if c.get(1).is_some() {
                            format!("Studio {}", c.get(2).unwrap().as_str())
                        } else {
                            c.get(2).unwrap().as_str().to_string()
                        }
                    })
                    .unwrap_or_else(|| "Unknown".to_string());

                if verbose {
                    println!("Found C function: {}: {}", name, category);
                }

                let (category, _) = categories.insert_full(category);

                let pre_marked = pre_marked.contains(name.as_str());
                functions.insert(
                    name,
                    CFunction {
                        category,
                        marked: pre_marked,
                    },
                );
            }
            _ => {}
        }
    }
    functions.sort_unstable_by(|_, c1, _, c2| c1.category.cmp(&c2.category));

    Ok(CInfo {
        functions,
        categories,
        enums,
        macros,
        structs,
    })
}
