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

    let mut seen = std::collections::HashSet::new();
    for entity in entities {
        if let Some(name) = entity.get_name() {
            if !seen.insert(name) {
                continue;
            }
        }

        match entity.get_kind() {
            clang::EntityKind::TypedefDecl | clang::EntityKind::MacroExpansion => {}
            clang::EntityKind::MacroDefinition => {
                let name = entity.get_name().unwrap();
                if macro_regex.is_match(&name) {
                    if verbose {
                        println!("Found Macro: {name}");
                    }

                    macros.insert(name, false);
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
                            Some((name, false))
                        }
                    })
                    .collect();
                enums.insert(name, CEnum { variants });
            }
            clang::EntityKind::StructDecl => {
                let name = entity.get_name().unwrap();
                structs.insert(name, false);
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

                functions.insert(
                    name,
                    CFunction {
                        category,
                        marked: false,
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
