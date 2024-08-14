// This is free and unencumbered software released into the public domain.

//! Output port arrays.

use crate::{
    prelude::{fmt, slice, AsRef, Deref, Index},
    Message, MessageSender, OutputPort, PortResult, System, Transport,
};

#[derive(Clone)]
pub struct OutputPorts<T: Message, const N: usize> {
    pub(crate) array: [OutputPort<T>; N],
}

impl<T: Message, const N: usize> OutputPorts<T, N> {
    pub const LEN: usize = N;

    pub fn new<X: Transport + Default>(system: &System<X>) -> Self {
        Self {
            array: [(); N].map(|_| OutputPort::new(system)),
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub const fn len(&self) -> usize {
        Self::LEN
    }

    pub const fn capacity(&self) -> usize {
        Self::LEN as _
    }

    #[must_use]
    pub fn get<I>(&self, index: usize) -> Option<&OutputPort<T>> {
        self.array.get(index)
    }

    pub fn iter(&self) -> slice::Iter<OutputPort<T>> {
        self.into_iter()
    }

    pub const fn as_slice(&self) -> &[OutputPort<T>] {
        self.array.as_slice()
    }
}

impl<T: Message, const N: usize> MessageSender<T> for OutputPorts<T, N> {
    fn send<'a>(&self, _message: impl Into<&'a T>) -> PortResult<()>
    where
        T: 'a,
    {
        todo!("OutputPorts::send") // TODO
    }
}

impl<T: Message, const N: usize> AsRef<[OutputPort<T>]> for OutputPorts<T, N> {
    fn as_ref(&self) -> &[OutputPort<T>] {
        self
    }
}

impl<T: Message, const N: usize> Deref for OutputPorts<T, N> {
    type Target = [OutputPort<T>];

    fn deref(&self) -> &Self::Target {
        self.array.as_slice()
    }
}

impl<T: Message, const N: usize> Index<usize> for OutputPorts<T, N> {
    type Output = OutputPort<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl<'a, T: Message + 'a, const N: usize> IntoIterator for &'a OutputPorts<T, N> {
    type Item = &'a OutputPort<T>;
    type IntoIter = slice::Iter<'a, OutputPort<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Message, const N: usize> fmt::Debug for OutputPorts<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(&self.array).finish()
    }
}
