// This is free and unencumbered software released into the public domain.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{self, Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};

pub(crate) fn expand_derive_block(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
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
        _ => panic!("`#[derive(Block)]` only supports structs"),
    };

    Ok(quote! {
        extern crate alloc;

        #[automatically_derived]
        #[allow(
            unused_qualifications,
            clippy::redundant_locals,
        )]
        impl #impl_generics ::protoflow::BlockDescriptor for #ident #ty_generics #where_clause {
            fn inputs(&self) -> ::alloc::vec::Vec<::protoflow::PortDescriptor> {
                ::alloc::vec![] // TODO
            }

            fn outputs(&self) -> ::alloc::vec::Vec<::protoflow::PortDescriptor> {
                ::alloc::vec![] // TODO
            }
        }
    })
}
