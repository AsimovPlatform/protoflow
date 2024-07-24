// This is free and unencumbered software released into the public domain.

//! This crate provides Protoflow's two derive macros.
//!
//! ```edition2021
//! # use protoflow::derive::{Block, System};
//! ```

extern crate proc_macro;

mod derives;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Block, attributes(input, output, parameter))]
pub fn derive_block(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input);
    derives::expand_derive_block(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(System, attributes(block))]
pub fn derive_system(input: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(input);
    derives::expand_derive_system(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
