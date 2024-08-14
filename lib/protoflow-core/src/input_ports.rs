// This is free and unencumbered software released into the public domain.

//! Input port arrays.

use crate::{
    prelude::{fmt, slice, AsRef, Deref, Index},
    InputPort, Message, System, Transport,
};

#[derive(Clone)]
pub struct InputPorts<T: Message, const N: usize> {
    pub(crate) array: [InputPort<T>; N],
}

impl<T: Message, const N: usize> InputPorts<T, N> {
    pub const LEN: usize = N;

    pub fn new<X: Transport + Default>(system: &System<X>) -> Self {
        Self {
            array: [(); N].map(|_| InputPort::new(system)),
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

    pub const fn as_slice(&self) -> &[InputPort<T>] {
        self.array.as_slice()
    }

    #[must_use]
    pub fn get<I>(&self, index: usize) -> Option<&InputPort<T>> {
        self.array.get(index)
    }

    pub fn iter(&self) -> slice::Iter<InputPort<T>> {
        self.into_iter()
    }
}

impl<T: Message, const N: usize> AsRef<[InputPort<T>]> for InputPorts<T, N> {
    fn as_ref(&self) -> &[InputPort<T>] {
        self
    }
}

impl<T: Message, const N: usize> Deref for InputPorts<T, N> {
    type Target = [InputPort<T>];

    fn deref(&self) -> &Self::Target {
        self.array.as_slice()
    }
}

impl<T: Message, const N: usize> Index<usize> for InputPorts<T, N> {
    type Output = InputPort<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[index]
    }
}

impl<'a, T: Message + 'a, const N: usize> IntoIterator for &'a InputPorts<T, N> {
    type Item = &'a InputPort<T>;
    type IntoIter = slice::Iter<'a, InputPort<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T: Message, const N: usize> fmt::Debug for InputPorts<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(&self.array).finish()
    }
}
