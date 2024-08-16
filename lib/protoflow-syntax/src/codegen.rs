// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::prelude::{fmt, String};
use quote::{format_ident, quote, ToTokens};
use sysml_parser::{ParsedBlock, ParsedImport, ParsedMember, ParsedModel};

#[derive(Debug, Default)]
pub struct Code(proc_macro2::TokenStream);

impl Code {
    pub fn unparse(&self) -> String {
        let file = syn::parse2::<syn::File>(self.0.clone()).unwrap();
        prettyplease::unparse(&file)
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl ToTokens for Code {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl From<&ParsedModel> for Code {
    fn from(model: &ParsedModel) -> Self {
        //std::eprintln!("model: {:#?}", model);
        let members = model.members().into_iter().map(Code::from);
        Code(quote! {
            fn main() {
                #(#members)*
            }
        })
    }
}

impl From<&ParsedMember> for Code {
    fn from(member: &ParsedMember) -> Self {
        use ParsedMember::*;
        match member {
            Import(import) => Code::from(import),
            Package(package) => {
                let members = package.members().into_iter().map(Code::from);
                Code(quote! {
                    #(#members)*
                })
            }
            BlockUsage(usage) => Code::from(usage),
            AttributeUsage(_usage) => Code::default(), // TODO
            PortUsage(_usage) => Code::default(),      // TODO
        }
    }
}

impl From<&ParsedImport> for Code {
    fn from(import: &ParsedImport) -> Self {
        let name = syn::Path {
            leading_colon: None,
            segments: import
                .imported_name
                .to_vec()
                .into_iter()
                .map(|s| syn::PathSegment {
                    ident: format_ident!("{}", s),
                    arguments: syn::PathArguments::None,
                })
                .collect(),
        };
        let code = quote! {
            use #name;
        };
        Self(code)
    }
}

impl From<&ParsedBlock> for Code {
    fn from(usage: &ParsedBlock) -> Self {
        let name = format_ident!(
            "{}",
            usage
                .name
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or_else(|| "block")
        );
        Self(quote! {
            let #name = s.block();
        })
    }
}
