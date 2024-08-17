// This is free and unencumbered software released into the public domain.

use crate::prelude::{String, ToString};

pub trait CoreBlocks {
    fn const_string(&self, value: impl ToString) -> Const<String>;
}

mod buffer;
pub use buffer::*;

mod r#const;
pub use r#const::*;

mod count;
pub use count::*;

mod delay;
pub use delay::*;

mod drop;
pub use drop::*;

mod random;
pub use random::*;
