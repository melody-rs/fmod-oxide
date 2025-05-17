// Copyright (c) 2024 Melody Madeline Lyons
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use itertools::Itertools;

use super::collect_c_info::CInfo;

const FMOD_OXIDE_DIR: &str = "fmod-oxide/src/";

pub fn mark(c_info: &mut CInfo, verbose: bool) -> color_eyre::Result<()> {
    for entry in walkdir::WalkDir::new(FMOD_OXIDE_DIR)
        .into_iter()
        .filter_ok(|entry| entry.path().extension().is_some_and(|ext| ext == "rs"))
    {
        let entry = std::fs::read_to_string(entry?.path())?;
        let file = syn::parse_file(&entry)?;
        for item in file.items {
            process_item(item, c_info, verbose)
        }
    }

    Ok(())
}

fn process_item(item: syn::Item, c_info: &mut CInfo, verbose: bool) {
    match item {
        syn::Item::Fn(item) => process_block(*item.block, c_info, verbose),
        syn::Item::Impl(item) => {
            for item in item.items.into_iter().filter_map(impl_item_into_fn) {
                process_block(item.block, c_info, verbose)
            }
        }
        syn::Item::Enum(item) => {
            for variant in item.variants {
                if let Some((_, expr)) = variant.discriminant {
                    process_expr(expr, c_info, verbose);
                }
            }
        }
        // we should NEVER run across a foreign item in the fmod-oxide crate
        syn::Item::Mod(item) => {
            if let Some((_, items)) = item.content {
                for item in items {
                    process_item(item, c_info, verbose)
                }
            }
        }
        syn::Item::Macro(item) => process_macro(item.mac, c_info, verbose),
        _ => {}
    }
}

fn process_block(block: syn::Block, c_info: &mut CInfo, verbose: bool) {
    for stmt in block.stmts {
        match stmt {
            syn::Stmt::Item(item) => process_item(item, c_info, verbose),
            syn::Stmt::Expr(expr, _) => process_expr(expr, c_info, verbose),
            syn::Stmt::Local(local) => {
                if let Some(init) = local.init {
                    process_expr(*init.expr, c_info, verbose);
                    if let Some((_, expr)) = init.diverge {
                        process_expr(*expr, c_info, verbose)
                    }
                }
            }
            syn::Stmt::Macro(stmt) => process_macro(stmt.mac, c_info, verbose),
        }
    }
}

fn process_macro(mac: syn::Macro, c_info: &mut CInfo, verbose: bool) {
    process_stream(mac.tokens, c_info, verbose);
}

fn process_stream(stream: proc_macro2::TokenStream, c_info: &mut CInfo, verbose: bool) {
    for tree in stream {
        match tree {
            proc_macro2::TokenTree::Group(group) => process_stream(group.stream(), c_info, verbose),
            proc_macro2::TokenTree::Ident(ident) => {
                let ident = ident.to_string();
                process_ident(ident, c_info, verbose);
            }
            _ => {}
        }
    }
}

fn process_ident(ident: String, c_info: &mut CInfo, verbose: bool) {
    if let Some(function) = c_info.functions.get_mut(&ident) {
        if verbose {
            println!("Found Rust calling C function: {ident}");
        }
        function.marked = true;
    }

    if let Some(marked) = c_info.macros.get_mut(&ident) {
        if verbose {
            println!("Found Rust mentioning C macro: {ident}");
        }
        *marked = true;
    }

    if let Some(marked) = c_info.structs.get_mut(&ident) {
        if verbose {
            println!("Found Rust mentioning C struct: {ident}");
        }
        *marked = true;
    }

    // save for last because this isnt a fast lookup
    for (_, c_enum) in c_info.enums.iter_mut() {
        if let Some(marked) = c_enum.variants.get_mut(&ident) {
            if verbose {
                println!("Found Rust mentioning C enum: {ident}");
            }
            *marked = true;
        }
    }
}

fn process_expr(expr: syn::Expr, c_info: &mut CInfo, verbose: bool) {
    match expr {
        syn::Expr::Array(expr) => {
            for elem in expr.elems {
                process_expr(elem, c_info, verbose)
            }
        }
        syn::Expr::Assign(expr) => {
            process_expr(*expr.left, c_info, verbose);
            process_expr(*expr.right, c_info, verbose)
        }
        syn::Expr::Async(expr) => process_block(expr.block, c_info, verbose),
        syn::Expr::Await(expr) => process_expr(*expr.base, c_info, verbose),
        syn::Expr::Binary(expr) => {
            process_expr(*expr.left, c_info, verbose);
            process_expr(*expr.right, c_info, verbose)
        }
        syn::Expr::Block(expr) => process_block(expr.block, c_info, verbose),
        syn::Expr::Break(expr) => {
            if let Some(expr) = expr.expr {
                process_expr(*expr, c_info, verbose)
            }
        }
        syn::Expr::Call(expr) => {
            process_expr(*expr.func, c_info, verbose);
            for arg in expr.args {
                process_expr(arg, c_info, verbose)
            }
        }
        syn::Expr::Cast(expr) => process_expr(*expr.expr, c_info, verbose),
        syn::Expr::Closure(expr) => process_expr(*expr.body, c_info, verbose),
        // don't need to process const blocks as none of the ffi functions are const
        // continue has no expr to process
        syn::Expr::Field(expr) => process_expr(*expr.base, c_info, verbose),
        syn::Expr::ForLoop(expr) => {
            process_expr(*expr.expr, c_info, verbose);
            process_block(expr.body, c_info, verbose);
        }
        // no clue what this is
        syn::Expr::Group(expr) => process_expr(*expr.expr, c_info, verbose),
        syn::Expr::If(expr) => {
            process_expr(*expr.cond, c_info, verbose);
            process_block(expr.then_branch, c_info, verbose);
            if let Some((_, expr)) = expr.else_branch {
                process_expr(*expr, c_info, verbose);
            }
        }
        syn::Expr::Index(expr) => {
            process_expr(*expr.expr, c_info, verbose);
            process_expr(*expr.index, c_info, verbose);
        }
        // infer has no expr to process
        syn::Expr::Let(expr) => process_expr(*expr.expr, c_info, verbose),
        // literal has no expr to process
        syn::Expr::Loop(expr) => process_block(expr.body, c_info, verbose),
        syn::Expr::Macro(expr) => {
            process_macro(expr.mac, c_info, verbose);
        }
        syn::Expr::Match(expr) => {
            process_expr(*expr.expr, c_info, verbose);
            for arm in expr.arms {
                if let Some((_, expr)) = arm.guard {
                    process_expr(*expr, c_info, verbose)
                }
                process_expr(*arm.body, c_info, verbose)
            }
        }
        syn::Expr::MethodCall(expr) => {
            // none of the c functions are methods, so we don't care about the method name
            process_expr(*expr.receiver, c_info, verbose);
            for arg in expr.args {
                process_expr(arg, c_info, verbose)
            }
        }
        syn::Expr::Paren(expr) => process_expr(*expr.expr, c_info, verbose),
        syn::Expr::Path(path) => {
            for segment in path.path.segments {
                let ident = segment.ident.to_string();
                process_ident(ident, c_info, verbose);
            }
        }
        syn::Expr::Range(expr) => {
            if let Some(start) = expr.start {
                process_expr(*start, c_info, verbose)
            }
            if let Some(end) = expr.end {
                process_expr(*end, c_info, verbose)
            }
        }
        syn::Expr::Reference(expr) => process_expr(*expr.expr, c_info, verbose),
        syn::Expr::Repeat(expr) => {
            process_expr(*expr.expr, c_info, verbose);
            process_expr(*expr.len, c_info, verbose)
        }
        syn::Expr::Return(expr) => {
            if let Some(expr) = expr.expr {
                process_expr(*expr, c_info, verbose)
            }
        }
        syn::Expr::Struct(expr) => {
            for field in expr.fields {
                process_expr(field.expr, c_info, verbose)
            }
            if let Some(expr) = expr.rest {
                process_expr(*expr, c_info, verbose)
            }
        }
        syn::Expr::Try(expr) => process_expr(*expr.expr, c_info, verbose),
        syn::Expr::TryBlock(expr) => process_block(expr.block, c_info, verbose),
        syn::Expr::Tuple(expr) => {
            for elem in expr.elems {
                process_expr(elem, c_info, verbose)
            }
        }
        syn::Expr::Unary(expr) => process_expr(*expr.expr, c_info, verbose),
        syn::Expr::Unsafe(expr) => process_block(expr.block, c_info, verbose),
        // can we process verbatim..?
        syn::Expr::While(expr) => {
            process_expr(*expr.cond, c_info, verbose);
            process_block(expr.body, c_info, verbose)
        }
        syn::Expr::Yield(expr) => {
            if let Some(expr) = expr.expr {
                process_expr(*expr, c_info, verbose)
            }
        }
        _ => {}
    }
}

fn impl_item_into_fn(item: syn::ImplItem) -> Option<syn::ImplItemFn> {
    if let syn::ImplItem::Fn(item) = item {
        Some(item)
    } else {
        None
    }
}
