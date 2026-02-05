//! AST transformations using syn
//!
//! This module handles:
//! - Splitting the generated code into types and client modules
//! - Removing types that are replaced with external crates
//! - Adding derives to error types

use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::{parse_quote, Attribute, File, Item, ItemImpl};

/// Types to remove (they're provided by external crates)
const TYPES_TO_REMOVE: &[&str] = &["AccountId", "NearGas", "NearToken", "CryptoHash"];

/// Types that are part of the error module we want to remove
const ERROR_MODULE_TYPES: &[&str] = &["ConversionError"];

/// Split the generated AST into types and client crates
pub fn split_and_transform(file: File) -> Result<(TokenStream, TokenStream)> {
    let mut types_items: Vec<Item> = Vec::new();
    let mut client_items: Vec<Item> = Vec::new();
    let mut use_statements: Vec<Item> = Vec::new();

    for item in file.items {
        match &item {
            // Use statements go to client
            Item::Use(_) => {
                use_statements.push(item);
            }
            // Types module gets split out
            Item::Mod(item_mod) if item_mod.ident == "types" => {
                let transformed = transform_types_module(item_mod.clone())?;
                types_items.extend(transformed);
            }
            // Client struct and impls go to client
            Item::Struct(s) if s.ident == "Client" => {
                client_items.push(item);
            }
            Item::Impl(i) => {
                // Check if this is impl for Client or ClientHooks
                if is_client_related_impl(i) {
                    client_items.push(item);
                } else if should_skip_impl(i) {
                    // Skip impls for external types (orphan rule)
                    continue;
                } else {
                    // Other impls might be for types
                    types_items.push(item);
                }
            }
            // Trait definitions like ClientHooks go to client
            Item::Trait(_) => {
                client_items.push(item);
            }
            // Everything else goes to client for now
            _ => {
                client_items.push(item);
            }
        }
    }

    // Build types crate tokens
    let types_tokens = build_types_crate(types_items);

    // Build client crate tokens
    let client_tokens = build_client_crate(use_statements, client_items);

    Ok((types_tokens, client_tokens))
}

fn is_client_related_impl(item: &ItemImpl) -> bool {
    if let syn::Type::Path(type_path) = &*item.self_ty {
        if let Some(segment) = type_path.path.segments.last() {
            let name = segment.ident.to_string();
            return name == "Client" || name.contains("ClientHooks");
        }
    }
    // Also check for `impl Trait for &Client`
    if let syn::Type::Reference(ref_type) = &*item.self_ty {
        if let syn::Type::Path(type_path) = &*ref_type.elem {
            if let Some(segment) = type_path.path.segments.last() {
                return segment.ident == "Client";
            }
        }
    }
    // Check the trait being implemented
    if let Some((_, path, _)) = &item.trait_ {
        if let Some(segment) = path.segments.last() {
            if segment.ident.to_string().contains("ClientHooks") {
                return true;
            }
        }
    }
    false
}

fn should_skip_impl(item: &ItemImpl) -> bool {
    // Skip impls for external types (would violate orphan rules)

    // Check self type
    if let syn::Type::Path(type_path) = &*item.self_ty {
        if let Some(segment) = type_path.path.segments.last() {
            let name = segment.ident.to_string();
            if TYPES_TO_REMOVE.contains(&name.as_str()) {
                return true;
            }
        }
        // Also check full path like ::std::string::String
        let path_str = type_path
            .path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect::<Vec<_>>()
            .join("::");
        if path_str.contains("String") || path_str == "u64" {
            // Check if this is a From impl involving our removed types
            if let Some((_, trait_path, _)) = &item.trait_ {
                if trait_path.segments.iter().any(|s| s.ident == "From") {
                    return true;
                }
            }
        }
    }

    // Check trait generics for From<RemovedType>
    if let Some((_, trait_path, _)) = &item.trait_ {
        for segment in &trait_path.segments {
            if segment.ident == "From" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    for arg in &args.args {
                        if let syn::GenericArgument::Type(syn::Type::Path(type_path)) = arg {
                            if let Some(seg) = type_path.path.segments.last() {
                                let name = seg.ident.to_string();
                                if TYPES_TO_REMOVE.contains(&name.as_str()) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    false
}

/// Transform the types module: remove replaced types, add derives
fn transform_types_module(types_mod: syn::ItemMod) -> Result<Vec<Item>> {
    let mut result = Vec::new();

    if let Some((_, items)) = types_mod.content {
        // First pass: collect types that have a manual Display impl
        let types_with_display = collect_types_with_display_impl(&items);

        for item in items {
            // Skip the error submodule
            if let Item::Mod(ref m) = item {
                if m.ident == "error" {
                    continue;
                }
                // Keep defaults module
                if m.ident == "defaults" {
                    result.push(item);
                    continue;
                }
            }

            // Skip types we're replacing
            if should_skip_item(&item) {
                continue;
            }

            // Skip impls that would violate orphan rules
            if let Item::Impl(ref i) = item {
                if should_skip_impl(i) {
                    continue;
                }
            }

            // Transform the item (add derives etc)
            let transformed = transform_item(item, &types_with_display);
            result.push(transformed);
        }
    }

    Ok(result)
}

/// Collect type names that have a manual Display impl
fn collect_types_with_display_impl(items: &[Item]) -> HashSet<String> {
    let mut result = HashSet::new();

    for item in items {
        if let Item::Impl(item_impl) = item {
            // Check if this is an impl for Display
            if let Some((_, trait_path, _)) = &item_impl.trait_ {
                let is_display = trait_path.segments.iter().any(|seg| {
                    seg.ident == "Display" || (seg.ident == "fmt" && trait_path.segments.len() > 1)
                });

                if is_display {
                    // Get the type name
                    if let syn::Type::Path(type_path) = &*item_impl.self_ty {
                        if let Some(segment) = type_path.path.segments.last() {
                            result.insert(segment.ident.to_string());
                        }
                    }
                }
            }
        }
    }

    result
}

fn should_skip_item(item: &Item) -> bool {
    let name = match item {
        Item::Struct(s) => s.ident.to_string(),
        Item::Enum(e) => e.ident.to_string(),
        Item::Type(t) => t.ident.to_string(),
        Item::Impl(i) => {
            // Skip impls for removed types
            if let syn::Type::Path(type_path) = &*i.self_ty {
                if let Some(segment) = type_path.path.segments.last() {
                    let name = segment.ident.to_string();
                    return TYPES_TO_REMOVE.contains(&name.as_str())
                        || ERROR_MODULE_TYPES.contains(&name.as_str());
                }
            }
            return false;
        }
        _ => return false,
    };

    TYPES_TO_REMOVE.contains(&name.as_str()) || ERROR_MODULE_TYPES.contains(&name.as_str())
}

fn transform_item(item: Item, types_with_display: &HashSet<String>) -> Item {
    match item {
        Item::Enum(mut e) => {
            let type_name = e.ident.to_string();
            // Add strum_macros::Display and thiserror::Error derive to error enums
            // but only if they don't already have a Display impl
            if type_name.ends_with("Error")
                && !type_name.starts_with("JsonRpcResponseFor")
                && !types_with_display.contains(&type_name)
            {
                add_error_derives(&mut e.attrs);
            }
            Item::Enum(e)
        }
        other => other,
    }
}

fn add_error_derives(attrs: &mut Vec<Attribute>) {
    // Add strum_macros::Display first (for Display impl), then thiserror::Error
    attrs.push(parse_quote!(#[derive(strum_macros::Display)]));
    attrs.push(parse_quote!(#[derive(thiserror::Error)]));
}

fn build_types_crate(items: Vec<Item>) -> TokenStream {
    quote! {
        //! This crate provides types for the Near OpenAPI specification.
        //!
        //! Used in [near-openapi-client](https://docs.rs/near-openapi-client/latest/near_openapi_client/)

        pub mod error;
        mod util;

        pub use near_account_id::AccountId;
        pub use near_gas::NearGas;
        pub use near_token::NearToken;
        pub use util::CryptoHash;

        #(#items)*
    }
}

fn build_client_crate(use_statements: Vec<Item>, items: Vec<Item>) -> TokenStream {
    quote! {
        //! NEAR Protocol JSON RPC API Client
        //!
        //! Generated from the NEAR OpenAPI specification.

        pub use near_openapi_types as types;

        #(#use_statements)*
        #(#items)*
    }
}
