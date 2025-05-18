// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::io::Write;
use std::path::PathBuf;

use color_eyre::owo_colors::OwoColorize;

mod collect_c_info;
mod mark_rust_references;

const WRAPPER_H_PATH: &str = "fmod-sys/src/wrapper.h";

pub fn coverage(
    core_include_dir: PathBuf,
    studio_include_dir: PathBuf,
    print: bool,
    verbose: bool,
) -> color_eyre::Result<()> {
    let clang = clang::Clang::new().unwrap();

    let index = clang::Index::new(&clang, false, true);
    let translation_unit = index
        .parser(WRAPPER_H_PATH)
        .arguments(&[
            "-I",
            core_include_dir.to_str().unwrap(),
            "-I",
            studio_include_dir.to_str().unwrap(),
        ])
        .detailed_preprocessing_record(true)
        .keep_going(true) // keep going even if we run into a fatal error (i.e. can't find stdbool)
        .parse()?;

    let mut c_info = collect_c_info::collect(&translation_unit, verbose)?;

    mark_rust_references::mark(&mut c_info, verbose)?;

    let minor = c_info.fmod_version & 0xFF;
    let major = (c_info.fmod_version >> 8) & 0xFF;
    let product = c_info.fmod_version >> 16;

    let mut coverage_md = std::fs::File::create(format!("COVERAGE.{product}.{major:0>2}.md"))?;
    let channel_filter_regex = regex::Regex::new(r"FMOD_(Channel|ChannelGroup)_(.*)$")?;
    let mut current_category = usize::MAX;

    let mut total_items = c_info.functions.len() + c_info.macros.len() + c_info.structs.len();
    total_items += c_info
        .enums
        .iter()
        .map(|(_, e)| e.variants.len())
        .sum::<usize>();

    let mut total_covered = 0;
    total_covered += c_info.functions.iter().filter(|(_, f)| f.marked).count();
    total_covered += c_info.macros.iter().filter(|(_, m)| **m).count();
    total_covered += c_info.structs.iter().filter(|(_, m)| **m).count();
    total_covered += c_info
        .enums
        .iter()
        .map(|(_, e)| e.variants.iter().filter(|(_, m)| **m).count())
        .sum::<usize>();

    writeln!(coverage_md, "# FMOD {product}.{major:0>2}.{minor:0>2}")?;

    writeln!(
        coverage_md,
        "Checked boxes mean that particular function/struct/enum/macro is mentioned in fmod-oxide."
    )?;
    writeln!(
        coverage_md,
        "It's a pretty decent metric for how much of FMOD this crate exposes."
    )?;

    writeln!(
        coverage_md,
        "Coverage: {total_covered}/{total_items} ({:.2}%)",
        total_covered as f32 / total_items as f32 * 100.0
    )?;

    writeln!(coverage_md, "# Functions")?;

    let fn_iter = c_info.functions.iter().filter(|(function, _)| {
        // check if relevant channel_control function exists, and remove it from the list
        if channel_filter_regex.is_match(function) {
            let channel_control_function =
                channel_filter_regex.replace(function, "FMOD_ChannelControl_$2");
            !c_info
                .functions
                .contains_key(channel_control_function.as_ref())
        } else {
            true
        }
    });
    for (name, function) in fn_iter {
        if function.category != current_category {
            current_category = function.category;
            let category = c_info.categories.get_index(function.category).unwrap();
            writeln!(coverage_md, "## {category}")?;
            if print {
                println!("{}", category.bright_yellow());
            }
        }
        if function.marked {
            writeln!(coverage_md, "- [x] `{name}`")?;
            if print {
                println!("{} ({})", name.bright_white(), "🗸".green());
            }
        } else {
            writeln!(coverage_md, "- [ ] `{name}`")?;
            if print {
                println!("{} ({})", name.bright_white(), "🗴".red())
            }
        }
    }

    writeln!(coverage_md, "# Structs")?;

    for (name, marked) in c_info.structs {
        if marked {
            writeln!(coverage_md, "- [x] `{name}`")?;
            if print {
                println!("{} ({})", name.bright_white(), "🗸".green());
            }
        } else {
            writeln!(coverage_md, "- [ ] `{name}`")?;
            if print {
                println!("{} ({})", name.bright_white(), "🗴".red())
            }
        }
    }

    writeln!(coverage_md, "# Enums")?;

    for (name, c_enum) in c_info.enums {
        writeln!(coverage_md, "## {name}")?;
        for (name, marked) in c_enum.variants {
            if marked {
                writeln!(coverage_md, "- [x] `{name}`")?;
                if print {
                    println!("{} ({})", name.bright_white(), "🗸".green());
                }
            } else {
                writeln!(coverage_md, "- [ ] `{name}`")?;
                if print {
                    println!("{} ({})", name.bright_white(), "🗴".red())
                }
            }
        }
    }

    writeln!(coverage_md, "# Macros")?;

    for (name, marked) in c_info.macros {
        if marked {
            writeln!(coverage_md, "- [x] `{name}`")?;
            if print {
                println!("{} ({})", name.bright_white(), "🗸".green());
            }
        } else {
            writeln!(coverage_md, "- [ ] `{name}`")?;
            if print {
                println!("{} ({})", name.bright_white(), "🗴".red())
            }
        }
    }

    coverage_md.flush()?;

    Ok(())
}
