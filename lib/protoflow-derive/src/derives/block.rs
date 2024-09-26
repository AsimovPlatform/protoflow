// This is free and unencumbered software released into the public domain.

use crate::util::protoflow_crate;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    self, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, FieldsUnnamed, Ident, Result,
};

pub(crate) fn expand_derive_block(input: &DeriveInput) -> Result<TokenStream> {
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named: fields, .. }),
            ..
        }) => expand_derive_block_for_struct(input, fields.into_iter().collect()),
        Data::Struct(DataStruct {
            fields:
                Fields::Unnamed(FieldsUnnamed {
                    unnamed: fields, ..
                }),
            ..
        }) => expand_derive_block_for_struct(input, fields.into_iter().collect()),
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => expand_derive_block_for_struct(input, Vec::new()),
        _ => panic!("`#[derive(Block)]` only supports structs"),
    }
}

pub(crate) fn expand_derive_block_for_struct(
    input: &DeriveInput,
    fields: Vec<&Field>,
) -> Result<TokenStream> {
    let protoflow = protoflow_crate();

    let ident = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields: Vec<(Ident, String)> = fields
        .iter()
        .filter_map(|field| {
            let Some(field_name) = &field.ident else {
                return None;
            };
            match &field.ty {
                syn::Type::Path(syn::TypePath { path, .. }) => {
                    let segment = path.segments.first().unwrap();
                    let ident = &segment.ident;
                    Some((field_name.clone(), ident.to_string()))
                }
                _ => return None,
            }
        })
        .collect();

    let port_names: Vec<Ident> = fields
        .iter()
        .filter(|(_, ty)| ty == "InputPort" || ty == "OutputPort") // FIXME: this is a hack
        .map(|(name, _)| name.clone())
        .collect();

    let input_port_names: Vec<Ident> = fields
        .iter()
        .filter(|(_, ty)| ty == "InputPort") // FIXME: this is a hack
        .map(|(name, _)| name.clone())
        .collect();

    let output_port_names: Vec<Ident> = fields
        .iter()
        .filter(|(_, ty)| ty == "OutputPort") // FIXME: this is a hack
        .map(|(name, _)| name.clone())
        .collect();

    let port_closes: Vec<TokenStream> = port_names
        .iter()
        .map(|port| {
            quote! {
                self.#port.close()?;
            }
        })
        .collect();

    let input_port_descriptors: Vec<TokenStream> = input_port_names
        .iter()
        .map(|port| {
            // TODO: mandatory name, ID; implement label, type
            let port_name = port.to_string();
            quote! {
                #protoflow::PortDescriptor {
                    direction: #protoflow::PortDirection::Input,
                    name: Some(#protoflow::prelude::String::from(#port_name)),
                    label: None,
                    r#type: None,
                    id: #protoflow::Port::id(&self.#port).unwrap(),
                    state: #protoflow::Port::state(&self.#port),
                }
            }
        })
        .collect();

    let output_port_descriptors: Vec<TokenStream> = output_port_names
        .iter()
        .map(|port| {
            // TODO: mandatory name, ID; implement label, type
            let port_name = port.to_string();
            quote! {
                #protoflow::PortDescriptor {
                    direction: #protoflow::PortDirection::Output,
                    name: Some(#protoflow::prelude::String::from(#port_name)),
                    label: None,
                    r#type: None,
                    id: #protoflow::Port::id(&self.#port).unwrap(),
                    state: #protoflow::Port::state(&self.#port),
                }
            }
        })
        .collect();

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
                #protoflow::prelude::vec![
                    #(#input_port_descriptors,)*
                ]
            }

            fn outputs(&self) -> #protoflow::prelude::Vec<#protoflow::PortDescriptor> {
                #protoflow::prelude::vec![
                    #(#output_port_descriptors,)*
                ]
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
                #(
                    #port_closes
                )*
                Ok(())
            }
        }
    };

    Ok(quote! {
        #impl_block_hooks
        #impl_block_descriptor
        #impl_sysml_traits
        #impl_dogma_traits
    })
}
