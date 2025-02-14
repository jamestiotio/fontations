//! Generating types from the opentype spec

use std::backtrace::Backtrace;

use log::debug;
use proc_macro2::TokenStream;
use quote::quote;

mod error;
mod fields;
mod flags_enums;
mod formatting;
mod parsing;
mod record;
mod table;

use parsing::{Item, Items, Phase};

pub use error::ErrorReport;

/// Codegeneration mode.
#[derive(Debug, Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// Generate parsing code
    Parse,
    /// Generate compilation code
    Compile,
}

pub fn generate_code(code_str: &str, mode: Mode) -> Result<String, syn::Error> {
    // Generation is done in phases (https://github.com/googlefonts/fontations/issues/71):
    // 1. Parse
    debug!("Parse (mode {:?})", mode);
    // This is the one step where we can't readily intercept the error with logged_syn_error
    let mut items: Items = syn::parse_str(code_str).map_err(|e| {
        debug!("{}", Backtrace::capture());
        e
    })?;
    items.sanity_check(Phase::Parse)?;

    // 2. Contemplate (semantic analysis)
    debug!("Analyze (mode {:?})", mode);
    items.resolve_pending()?;
    items.sanity_check(Phase::Analysis)?;

    // 3. Generate
    debug!("Generate (mode {:?})", mode);
    let tables = match &mode {
        Mode::Parse => generate_parse_module(&items),
        Mode::Compile => generate_compile_module(&items),
    }?;

    // 4. Touchup
    debug!("Touchup (mode {:?})", mode);
    let source_str = formatting::format(tables)?;

    Ok(format!(
        "\
    // THIS FILE IS AUTOGENERATED.\n\
    // Any changes to this file will be overwritten.\n\
    // For more information about how codegen works, see font-codegen/README.md\n\n\
    {source_str}",
    ))
}

pub(crate) fn generate_parse_module(items: &Items) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut code = Vec::new();
    for item in items.iter() {
        let item_code = match item {
            Item::Record(item) => record::generate(item, items)?,
            Item::Table(item) => table::generate(item)?,
            Item::GenericGroup(item) => table::generate_group(item)?,
            Item::Format(item) => table::generate_format_group(item)?,
            Item::RawEnum(item) => flags_enums::generate_raw_enum(item),
            Item::Flags(item) => flags_enums::generate_flags(item),
            Item::Extern(..) => Default::default(),
        };
        code.push(item_code);
    }

    Ok(quote! {
        #[allow(unused_imports)]
        use crate::codegen_prelude::*;
        #(#code)*
    })
}

pub(crate) fn generate_compile_module(
    items: &Items,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let code = items
        .iter()
        .map(|item| match item {
            Item::Record(item) => record::generate_compile(item, &items.parse_module_path),
            Item::Table(item) => table::generate_compile(item, &items.parse_module_path),
            Item::GenericGroup(item) => {
                table::generate_group_compile(item, &items.parse_module_path)
            }
            Item::Format(item) => table::generate_format_compile(item, items),
            Item::RawEnum(item) => Ok(flags_enums::generate_raw_enum_compile(item)),
            Item::Flags(item) => Ok(flags_enums::generate_flags_compile(item)),
            Item::Extern(..) => Ok(TokenStream::new()),
        })
        .collect::<Result<Vec<_>, _>>()?;

    let import_from_parse_mod = items.iter().filter_map(|item| match item {
        Item::Flags(item) => Some(&item.name),
        Item::RawEnum(item) => Some(&item.name),
        _ => None,
    });
    let parse_mod_path = &items.parse_module_path;

    Ok(quote! {
        #[allow(unused_imports)]
        use crate::codegen_prelude::*;
        pub use #parse_mod_path::{ #( #import_from_parse_mod, )* };

        #( #code )*
    })
}

impl std::str::FromStr for Mode {
    type Err = miette::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "parse" => Ok(Self::Parse),
            "compile" => Ok(Self::Compile),
            other => Err(miette::Error::msg(format!(
                "expected one of 'parse' or 'compile' (found {other})"
            ))),
        }
    }
}
