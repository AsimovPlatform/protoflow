// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
extern crate std;

use crate::{
    prelude::{vec, BTreeSet, Vec},
    AnalysisError, AnalysisResult,
};
use error_stack::ResultExt;
use protoflow_blocks::BLOCKS;
use sysml_model::QualifiedName;
use sysml_parser::{ParsedBlock, ParsedMember, ParsedModel};

pub use sysml_parser::ParseError;

#[derive(Debug, Default)]
pub struct SystemParser {
    pub(crate) model: ParsedModel,
    pub(crate) imported_names: BTreeSet<QualifiedName>,
}

impl SystemParser {
    fn new(model: ParsedModel) -> Self {
        Self {
            model,
            ..Default::default()
        }
    }

    #[cfg(feature = "std")]
    pub fn from_file(pathname: impl AsRef<std::path::Path>) -> AnalysisResult<Self> {
        Ok(Self::new(
            sysml_parser::parse_from_file(pathname).change_context(AnalysisError::ParseFailure)?,
        ))
    }

    #[cfg(feature = "std")]
    pub fn from_reader(reader: impl std::io::Read) -> AnalysisResult<Self> {
        Ok(Self::new(
            sysml_parser::parse_from_reader(reader).change_context(AnalysisError::ParseFailure)?,
        ))
    }

    pub fn from_string(&mut self, input: &str) -> AnalysisResult<Self> {
        Ok(Self::new(
            sysml_parser::parse_from_string(input).change_context(AnalysisError::ParseFailure)?,
        ))
    }

    pub fn check(&mut self) -> AnalysisResult<()> {
        let members: Vec<ParsedMember> = self.model.members().iter().cloned().collect();
        for member in members {
            self.check_usage(&member)?;
        }
        Ok(())
    }

    pub fn check_usage(
        &mut self,
        member: &ParsedMember,
    ) -> core::result::Result<(), AnalysisError> {
        match member {
            ParsedMember::Import(import) => match import.imported_name.to_tuple3() {
                (Some("Protoflow"), Some("*") | Some("**"), None) => {
                    for block_name in BLOCKS.iter() {
                        self.imported_names.insert(QualifiedName::new(vec![
                            "Protoflow".into(),
                            (*block_name).into(),
                        ]));
                    }
                }
                (Some("Protoflow"), Some(unqualified_name), None) => {
                    if !BLOCKS
                        .iter()
                        .any(|block_name| *block_name == unqualified_name)
                    {
                        return Err(AnalysisError::InvalidImport(import.imported_name.clone()));
                    }
                    self.imported_names.insert(import.imported_name.clone());
                }
                _ => {
                    return Err(AnalysisError::InvalidImport(import.imported_name.clone()));
                }
            },
            ParsedMember::Package(package) => {
                for member in package.members() {
                    self.check_usage(&member)?;
                }
            }
            ParsedMember::BlockUsage(block) => {
                if let Some(definition_name) = &block.definition {
                    if !self.imported_names.contains(&definition_name) {
                        return Err(AnalysisError::UnknownName(definition_name.clone()));
                    }
                }
                let _ = self.check_block_usage(block)?;
            }
            ParsedMember::AttributeUsage(attribute) => {
                if let Some(definition_name) = &attribute.definition {
                    if !self.imported_names.contains(&definition_name) {
                        return Err(AnalysisError::UnknownName(definition_name.clone()));
                    }
                }
            }
            ParsedMember::PortUsage(_port) => {
                unreachable!("PortUsage")
            }
        };
        Ok(())
    }

    pub fn check_block_usage(
        &mut self,
        _member: &ParsedBlock,
    ) -> core::result::Result<(), AnalysisError> {
        Ok(()) // TODO
    }
}
