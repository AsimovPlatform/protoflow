// This is free and unencumbered software released into the public domain.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{self, Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};

pub(crate) fn expand_derive_system(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    let ident = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let _fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named: fields, .. }),
            ..
        }) => fields.into_iter().collect(),
        Data::Struct(DataStruct {
            fields:
                Fields::Unnamed(FieldsUnnamed {
                    unnamed: fields, ..
                }),
            ..
        }) => fields.into_iter().collect(),
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => Vec::new(),
        _ => panic!("`#[derive(System)]` only supports structs"),
    };

    Ok(quote! {
        #[automatically_derived]
        #[allow(
            unused_qualifications,
            clippy::redundant_locals,
        )]
        impl #impl_generics ::protoflow::System for #ident #ty_generics #where_clause {}
    })
}
