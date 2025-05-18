// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use itertools::Itertools;
use syn::visit::Visit;

use super::collect_c_info::CInfo;

const FMOD_OXIDE_DIR: &str = "fmod-oxide/src/";

struct Visitor<'a> {
    c_info: &'a mut CInfo,
    verbose: bool,
}

pub fn mark(c_info: &mut CInfo, verbose: bool) -> color_eyre::Result<()> {
    for entry in walkdir::WalkDir::new(FMOD_OXIDE_DIR)
        .into_iter()
        .filter_ok(|entry| entry.path().extension().is_some_and(|ext| ext == "rs"))
    {
        let entry = std::fs::read_to_string(entry?.path())?;
        let file = syn::parse_file(&entry)?;
        Visitor { c_info, verbose }.visit_file(&file);
    }

    Ok(())
}

impl<'ast, 'info> Visit<'ast> for Visitor<'info> {
    fn visit_ident(&mut self, i: &'ast proc_macro2::Ident) {
        let ident = i.to_string();
        if let Some(function) = self.c_info.functions.get_mut(&ident) {
            if self.verbose {
                println!("Found Rust calling C function: {ident}");
            }
            function.marked = true;
        }

        if let Some(marked) = self.c_info.macros.get_mut(&ident) {
            if self.verbose {
                println!("Found Rust mentioning C macro: {ident}");
            }
            *marked = true;
        }

        if let Some(marked) = self.c_info.structs.get_mut(&ident) {
            if self.verbose {
                println!("Found Rust mentioning C struct: {ident}");
            }
            *marked = true;
        }

        // save for last because this isnt a fast lookup
        for (_, c_enum) in self.c_info.enums.iter_mut() {
            if let Some(marked) = c_enum.variants.get_mut(&ident) {
                if self.verbose {
                    println!("Found Rust mentioning C enum: {ident}");
                }
                *marked = true;
            }
        }
    }

    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        self.process_stream(i.tokens.clone());
    }
}

impl Visitor<'_> {
    fn process_stream(&mut self, stream: proc_macro2::TokenStream) {
        for tree in stream {
            match tree {
                proc_macro2::TokenTree::Group(group) => self.process_stream(group.stream()),
                proc_macro2::TokenTree::Ident(ident) => {
                    self.visit_ident(&ident);
                }
                _ => {}
            }
        }
    }
}
