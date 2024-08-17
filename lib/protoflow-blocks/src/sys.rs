// This is free and unencumbered software released into the public domain.

pub trait SysBlocks {
    fn read_dir(&self) -> ReadDir;
    fn read_env(&self) -> ReadEnv;
    fn read_file(&self) -> ReadFile;
    fn read_stdin(&self) -> ReadStdin;
    fn write_file(&self) -> WriteFile;
    fn write_stderr(&self) -> WriteStderr;
    fn write_stdout(&self) -> WriteStdout;
}

mod read_dir;
pub use read_dir::*;

mod read_env;
pub use read_env::*;

mod read_file;
pub use read_file::*;

mod read_stdin;
pub use read_stdin::*;

mod write_file;
pub use write_file::*;

mod write_stderr;
pub use write_stderr::*;

mod write_stdout;
pub use write_stdout::*;
