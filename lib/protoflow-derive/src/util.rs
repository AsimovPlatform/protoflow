// This is free and unencumbered software released into the public domain.

use proc_macro2::{Span, TokenStream};
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::Ident;

pub(crate) fn protoflow_crate() -> TokenStream {
    let found_crate = crate_name("protoflow").expect("protoflow is present in `Cargo.toml`");
    match found_crate {
        FoundCrate::Itself => quote!(crate),
        FoundCrate::Name(name) => {
            let ident = Ident::new(&name, Span::call_site());
            quote!(#ident)
        }
    }
}
