// This is free and unencumbered software released into the public domain.

extern crate std;

use crate::{
    prelude::{fmt, String, Vec},
    AnalysisError,
};
use error_stack::Report;
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

impl TryFrom<&ParsedModel> for Code {
    type Error = Report<AnalysisError>;

    fn try_from(model: &ParsedModel) -> Result<Self, Self::Error> {
        let members = model
            .members()
            .into_iter()
            .map(|member| Code::try_from(member))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Code(quote! {
            fn main() {
                #(#members)*
            }
        }))
    }
}

impl TryFrom<&ParsedMember> for Code {
    type Error = Report<AnalysisError>;

    fn try_from(member: &ParsedMember) -> Result<Self, Self::Error> {
        use ParsedMember::*;
        match member {
            Import(import) => Code::try_from(import),
            Package(package) => {
                let members = package
                    .members()
                    .into_iter()
                    .map(|member| Code::try_from(member))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Code(quote! {
                    #(#members)*
                }))
            }
            BlockUsage(usage) => Code::try_from(usage),
            AttributeUsage(_usage) => Ok(Code::default()), // TODO
            PortUsage(_usage) => Ok(Code::default()),      // TODO
        }
    }
}

impl TryFrom<&ParsedImport> for Code {
    type Error = Report<AnalysisError>;

    fn try_from(import: &ParsedImport) -> Result<Self, Self::Error> {
        Ok(Self(match import.imported_name.to_tuple3() {
            (Some("Protoflow"), Some("*") | Some("**"), None) => {
                quote! {
                    use protoflow::*;
                }
            }
            (Some("Protoflow"), Some(unqualified_name), None) => {
                let block_name = format_ident!("{}", unqualified_name);
                quote! {
                    use protoflow::blocks::#block_name;
                }
            }
            _ => {
                return Err(Report::new(AnalysisError::InvalidImport(
                    import.imported_name.clone(),
                )));
            }
        }))
    }
}

impl TryFrom<&ParsedBlock> for Code {
    type Error = Report<AnalysisError>;

    fn try_from(usage: &ParsedBlock) -> Result<Self, Self::Error> {
        let name = format_ident!(
            "{}",
            usage
                .name
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or_else(|| "block")
        );
        Ok(Self(quote! {
            let #name = s.block();
        }))
    }
}
