// This is free and unencumbered software released into the public domain.

//! This crate provides Protoflow's derive macros.
//!
//! ```edition2021
//! # use protoflow_derive::{Block, FunctionBlock, Subsystem, System};
//! ```

#![deny(unsafe_code)]

extern crate proc_macro;

mod derives;
pub(crate) mod util;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Block, attributes(input, output, parameter, state))]
pub fn derive_block(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    derives::expand_derive_block(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(FunctionBlock, attributes())]
pub fn derive_function_block(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    derives::expand_derive_function_block(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Subsystem, attributes(block))]
pub fn derive_subsystem(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    derives::expand_derive_system(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(System, attributes(block))]
pub fn derive_system(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    derives::expand_derive_system(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
