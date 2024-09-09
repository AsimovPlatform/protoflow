// This is free and unencumbered software released into the public domain.

use crate::util::protoflow_crate;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{self, Data, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Result};

pub(crate) fn expand_derive_function_block(input: &DeriveInput) -> Result<TokenStream> {
    let protoflow = protoflow_crate();

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

    let impl_dogma_traits = quote! {
        #[automatically_derived]
        #[allow(
            unused_qualifications,
            clippy::redundant_locals,
        )]
        impl #impl_generics #protoflow::prelude::MaybeNamed for #ident #ty_generics #where_clause {
            fn name(&self) -> #protoflow::prelude::Option<#protoflow::prelude::Cow<str>> {
                None // TODO
            }
        }

        #[automatically_derived]
        #[allow(
            unused_qualifications,
            clippy::redundant_locals,
        )]
        impl #impl_generics #protoflow::prelude::MaybeLabeled for #ident #ty_generics #where_clause {
            fn label(&self) -> #protoflow::prelude::Option<#protoflow::prelude::Cow<str>> {
                None // TODO
            }
        }
    };

    #[cfg(not(feature = "sysml"))]
    let impl_sysml_traits = quote! {};

    #[cfg(feature = "sysml")]
    let impl_sysml_traits = quote! {
        #[automatically_derived]
        #[allow(
            unused_qualifications,
            clippy::redundant_locals,
        )]
        impl #impl_generics #protoflow::prelude::sysml_model::BlockDefinition for #ident #ty_generics #where_clause {}
        impl #impl_generics #protoflow::prelude::sysml_model::PartDefinition for #ident #ty_generics #where_clause {}
        impl #impl_generics #protoflow::prelude::sysml_model::ItemDefinition for #ident #ty_generics #where_clause {}
        impl #impl_generics #protoflow::prelude::sysml_model::OccurrenceDefinition for #ident #ty_generics #where_clause {}
        impl #impl_generics #protoflow::prelude::sysml_model::Definition for #ident #ty_generics #where_clause {}
        impl #impl_generics #protoflow::prelude::sysml_model::Structure for #ident #ty_generics #where_clause {}
        impl #impl_generics #protoflow::prelude::sysml_model::Class for #ident #ty_generics #where_clause {}
        impl #impl_generics #protoflow::prelude::sysml_model::Classifier for #ident #ty_generics #where_clause {}
        impl #impl_generics #protoflow::prelude::sysml_model::Type for #ident #ty_generics #where_clause {}
        impl #impl_generics #protoflow::prelude::sysml_model::Namespace for #ident #ty_generics #where_clause {}
        impl #impl_generics #protoflow::prelude::sysml_model::Element for #ident #ty_generics #where_clause {}
    };

    let impl_block_descriptor = quote! {
        #[automatically_derived]
        #[allow(
            unused_qualifications,
            clippy::redundant_locals,
        )]
        impl #impl_generics #protoflow::BlockDescriptor for #ident #ty_generics #where_clause {
            fn inputs(&self) -> #protoflow::prelude::Vec<#protoflow::PortDescriptor> {
                #protoflow::prelude::vec![#protoflow::PortDescriptor::from(&self.0)]
            }

            fn outputs(&self) -> #protoflow::prelude::Vec<#protoflow::PortDescriptor> {
                #protoflow::prelude::vec![#protoflow::PortDescriptor::from(&self.1)]
            }
        }
    };

    let impl_block_hooks = quote! {
        #[automatically_derived]
        #[allow(
            unused_qualifications,
            clippy::redundant_locals,
        )]
        impl #impl_generics #protoflow::BlockHooks for #ident #ty_generics #where_clause {
            fn pre_execute(&mut self, _runtime: &dyn #protoflow::BlockRuntime) -> #protoflow::BlockResult {
                Ok(())
            }

            fn post_execute(&mut self, _runtime: &dyn #protoflow::BlockRuntime) -> #protoflow::BlockResult {
                self.0.close()?;
                self.1.close()?;
                Ok(())
            }
        }
    };

    let impl_block_execute = quote! {
        #[automatically_derived]
        #[allow(
            unused_qualifications,
            clippy::redundant_locals,
        )]
        impl #impl_generics #protoflow::Block for #ident #ty_generics #where_clause {
            fn execute(&mut self, _runtime: &dyn #protoflow::BlockRuntime) -> #protoflow::BlockResult {
                let input = &self.0;
                let output = &self.1;
                while let Some(message) = #protoflow::InputPort::recv(input)? {
                    if #protoflow::Port::is_connected(output) {
                        let result = #protoflow::FunctionBlock::compute(self, message)?;
                        #protoflow::OutputPort::send(output, &result)?;
                    }
                }
                Ok(())
            }
        }
    };

    Ok(quote! {
        #impl_block_execute
        #impl_block_hooks
        #impl_block_descriptor
        #impl_sysml_traits
        #impl_dogma_traits
    })
}
