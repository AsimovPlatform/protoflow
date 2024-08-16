// This is free and unencumbered software released into the public domain.

#![allow(unused)]

pub type Result = std::result::Result<(), Sysexits>;

#[allow(non_camel_case_types, dead_code)]
#[derive(Copy, Clone, Debug, Default)]
#[repr(u8)]
pub enum Sysexits {
    #[default]
    EX_OK = 0,
    EX_USAGE = 64,
    EX_DATAERR = 65,
    EX_NOINPUT = 66,
    EX_NOUSER = 67,
    EX_NOHOST = 68,
    EX_UNAVAILABLE = 69,
    EX_SOFTWARE = 70,
    EX_OSERR = 71,
    EX_OSFILE = 72,
    EX_CANTCREAT = 73,
    EX_IOERR = 74,
    EX_TEMPFAIL = 75,
    EX_PROTOCOL = 76,
    EX_NOPERM = 77,
    EX_CONFIG = 78,
}

impl std::process::Termination for Sysexits {
    fn report(self) -> std::process::ExitCode {
        (self as u8).into()
    }
}

impl From<std::boxed::Box<dyn std::error::Error>> for Sysexits {
    fn from(error: std::boxed::Box<dyn std::error::Error>) -> Self {
        std::eprintln!("{}: {:?}", "protoflow", error);
        Self::EX_SOFTWARE
    }
}

impl From<std::io::Error> for Sysexits {
    fn from(error: std::io::Error) -> Self {
        use std::io::ErrorKind::*;
        std::eprintln!("{}: {:?}", "protoflow", error);
        match error.kind() {
            AddrInUse => Self::EX_TEMPFAIL,
            AddrNotAvailable => Self::EX_USAGE,
            AlreadyExists => Self::EX_CANTCREAT,
            BrokenPipe => Self::EX_IOERR,
            ConnectionAborted => Self::EX_PROTOCOL,
            ConnectionRefused => Self::EX_UNAVAILABLE,
            ConnectionReset => Self::EX_PROTOCOL,
            Interrupted => Self::EX_TEMPFAIL,
            InvalidData => Self::EX_DATAERR,
            InvalidInput => Self::EX_DATAERR,
            NotConnected => Self::EX_PROTOCOL,
            NotFound => Self::EX_NOINPUT,
            Other => Self::EX_UNAVAILABLE,
            OutOfMemory => Self::EX_TEMPFAIL,
            PermissionDenied => Self::EX_NOPERM,
            TimedOut => Self::EX_IOERR,
            UnexpectedEof => Self::EX_IOERR,
            Unsupported => Self::EX_SOFTWARE,
            WouldBlock => Self::EX_IOERR,
            WriteZero => Self::EX_IOERR,
            _ => Self::EX_UNAVAILABLE, // catch-all
        }
    }
}

impl From<protoflow_syntax::ParseError> for Sysexits {
    fn from(error: protoflow_syntax::ParseError) -> Self {
        std::eprintln!("{}: {:?}", "protoflow", error);
        Self::EX_NOINPUT
    }
}

impl From<error_stack::Report<protoflow_syntax::AnalysisError>> for Sysexits {
    fn from(error: error_stack::Report<protoflow_syntax::AnalysisError>) -> Self {
        use protoflow_syntax::AnalysisError::*;
        std::eprintln!("{}: {:?}", "protoflow", error); // TODO: pretty print it
        match error.current_context() {
            ParseFailure => Self::EX_NOINPUT,
            InvalidImport(_) => Self::EX_DATAERR,
            UnknownName(_) => Self::EX_DATAERR,
            Other(_) => Self::EX_SOFTWARE,
        }
    }
}

impl From<crate::commands::check::CheckError> for Sysexits {
    fn from(error: crate::commands::check::CheckError) -> Self {
        use crate::commands::check::CheckError::*;
        std::eprintln!("{}: {:?}", "protoflow", error);
        match error {
            _ => Self::EX_SOFTWARE, // TODO
        }
    }
}

impl From<crate::commands::execute::ExecuteError> for Sysexits {
    fn from(error: crate::commands::execute::ExecuteError) -> Self {
        use crate::commands::execute::ExecuteError::*;
        std::eprintln!("{}: {:?}", "protoflow", error);
        match error {
            MissingParameter(_) => Self::EX_USAGE,
            InvalidParameter(_) => Self::EX_USAGE,
            UnknownSystem(_) => Self::EX_UNAVAILABLE,
        }
    }
}

impl From<crate::commands::generate::GenerateError> for Sysexits {
    fn from(error: crate::commands::generate::GenerateError) -> Self {
        use crate::commands::generate::GenerateError::*;
        std::eprintln!("{}: {:?}", "protoflow", error);
        match error {
            _ => Self::EX_SOFTWARE, // TODO
        }
    }
}

pub fn exit(code: Sysexits) -> ! {
    std::process::exit(code as i32);
}

macro_rules! abort {
    ($code:expr, $($t:tt)*) => {{
        std::eprintln!($($t)*);
        exit($code)
    }};
}
