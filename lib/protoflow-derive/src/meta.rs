// This is free and unencumbered software released into the public domain.

use syn::Attribute;

#[derive(Debug, Clone)]
pub enum BlockFieldAttribute {
    Input,
    Output,
    Parameter,
    State,
}

impl TryFrom<&Attribute> for BlockFieldAttribute {
    type Error = ();

    fn try_from(attr: &Attribute) -> Result<Self, Self::Error> {
        let path = attr.path();
        if path.is_ident("input") {
            Ok(Self::Input)
        } else if path.is_ident("output") {
            Ok(Self::Output)
        } else if path.is_ident("parameter") {
            Ok(Self::Parameter)
        } else if path.is_ident("state") {
            Ok(Self::State)
        } else {
            Err(())
        }
    }
}
