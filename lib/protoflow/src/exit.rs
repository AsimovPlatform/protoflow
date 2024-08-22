// This is free and unencumbered software released into the public domain.

use clientele::SysexitsError;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExitCode(pub SysexitsError);

impl core::fmt::Display for ExitCode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<std::boxed::Box<dyn std::error::Error>> for ExitCode {
    fn from(error: std::boxed::Box<dyn std::error::Error>) -> Self {
        std::eprintln!("{}: {:?}", "protoflow", error);
        Self(SysexitsError::from(error))
    }
}

impl From<std::io::Error> for ExitCode {
    fn from(error: std::io::Error) -> Self {
        std::eprintln!("{}: {:?}", "protoflow", error);
        Self(SysexitsError::from(error))
    }
}

impl From<error_stack::Report<protoflow_syntax::AnalysisError>> for ExitCode {
    fn from(error: error_stack::Report<protoflow_syntax::AnalysisError>) -> Self {
        use protoflow_syntax::AnalysisError::*;
        std::eprintln!("{}: {:?}", "protoflow", error); // TODO: pretty print it
        match error.current_context() {
            ParseFailure => Self(SysexitsError::EX_NOINPUT),
            InvalidImport(_) => Self(SysexitsError::EX_DATAERR),
            UnknownName(_) => Self(SysexitsError::EX_DATAERR),
            Other(_) => Self(SysexitsError::EX_SOFTWARE),
        }
    }
}

impl From<protoflow_blocks::StdioError> for ExitCode {
    fn from(error: protoflow_blocks::StdioError) -> Self {
        use protoflow_blocks::StdioError::*;
        std::eprintln!("{}: {}", "protoflow", error);
        match error {
            UnknownSystem(_) => Self(SysexitsError::EX_UNAVAILABLE),
            MissingParameter(_) => Self(SysexitsError::EX_USAGE),
            InvalidParameter(_) => Self(SysexitsError::EX_USAGE),
        }
    }
}

impl From<protoflow_syntax::ParseError> for ExitCode {
    fn from(error: protoflow_syntax::ParseError) -> Self {
        std::eprintln!("{}: {:?}", "protoflow", error);
        Self(SysexitsError::EX_NOINPUT)
    }
}

#[cfg(feature = "beta")]
impl From<crate::commands::check::CheckError> for ExitCode {
    fn from(error: crate::commands::check::CheckError) -> Self {
        use crate::commands::check::CheckError::*;
        std::eprintln!("{}: {:?}", "protoflow", error);
        match error {
            _ => Self(SysexitsError::EX_SOFTWARE), // TODO
        }
    }
}

impl From<crate::commands::execute::ExecuteError> for ExitCode {
    fn from(error: crate::commands::execute::ExecuteError) -> Self {
        use crate::commands::execute::ExecuteError::*;
        std::eprintln!("{}: {}", "protoflow", error);
        match error {
            UnknownSystem(_) => Self(SysexitsError::EX_UNAVAILABLE),
            MissingParameter(_) => Self(SysexitsError::EX_USAGE),
            InvalidParameter(_) => Self(SysexitsError::EX_USAGE),
            InvalidEncoding(_) => Self(SysexitsError::EX_USAGE),
        }
    }
}

#[cfg(feature = "beta")]
impl From<crate::commands::generate::GenerateError> for ExitCode {
    fn from(error: crate::commands::generate::GenerateError) -> Self {
        use crate::commands::generate::GenerateError::*;
        std::eprintln!("{}: {:?}", "protoflow", error);
        match error {
            _ => Self(SysexitsError::EX_SOFTWARE), // TODO
        }
    }
}

impl std::process::Termination for ExitCode {
    fn report(self) -> std::process::ExitCode {
        self.0.report()
    }
}

impl std::error::Error for ExitCode {}

impl error_stack::Context for ExitCode {}
