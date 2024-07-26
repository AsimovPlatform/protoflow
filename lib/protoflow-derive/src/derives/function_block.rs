// This is free and unencumbered software released into the public domain.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{self, Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed};

pub(crate) fn expand_derive_function_block(input: &DeriveInput) -> Result<TokenStream, syn::Error> {
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
        _ => panic!("`#[derive(FunctionBlock)]` only supports structs"),
    };

    Ok(quote! {
        #[automatically_derived]
        #[allow(
            unused_qualifications,
            clippy::redundant_locals,
        )]
        impl #impl_generics protoflow::BlockDescriptor for #ident #ty_generics #where_clause {
            fn inputs(&self) -> protoflow::prelude::Vec<protoflow::PortDescriptor> {
                protoflow::prelude::vec![protoflow::PortDescriptor::from(&self.0)]
            }

            fn outputs(&self) -> protoflow::prelude::Vec<protoflow::PortDescriptor> {
                protoflow::prelude::vec![protoflow::PortDescriptor::from(&self.1)]
            }
        }

        #[automatically_derived]
        #[allow(
            unused_qualifications,
            clippy::redundant_locals,
        )]
        impl #impl_generics protoflow::Block for #ident #ty_generics #where_clause {
            fn execute(&mut self, runtime: &dyn protoflow::Runtime) -> protoflow::prelude::Result<(), protoflow::BlockError> {
                let input = &self.0;
                let output = &self.1;
                while let Some(message) = protoflow::InputPort::receive(input)? {
                    if protoflow::Port::is_connected(output) {
                        let result = protoflow::FunctionBlock::compute(self, message)?;
                        protoflow::OutputPort::send(output, &result)?;
                    }
                }
                Ok(())
            }
        }
    })
}
