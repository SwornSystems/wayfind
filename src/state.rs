use alloc::boxed::Box;
use alloc::string::String;
use core::fmt;
use core::num::NonZeroUsize;

use crate::node::Data;

/// Root node of the tree.
#[derive(Clone, Debug)]
pub(crate) struct RootState;

impl RootState {
    pub(crate) const fn new() -> Self {
        Self
    }
}

impl fmt::Display for RootState {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

/// A static byte prefix.
#[derive(Clone, Debug)]
pub(crate) struct StaticState {
    /// Might not be valid UTF-8 due to multibyte splitting.
    pub prefix: Box<[u8]>,
}

impl StaticState {
    pub(crate) fn new(prefix: &[u8]) -> Self {
        Self {
            prefix: prefix.into(),
        }
    }
}

impl fmt::Display for StaticState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.prefix))
    }
}

/// A dynamic parameter.
#[derive(Clone, Debug)]
pub(crate) struct DynamicState {
    pub id: Option<NonZeroUsize>,
    pub name: Box<str>,
}

impl DynamicState {
    pub(crate) fn new(name: &str) -> Self {
        Self {
            id: None,
            name: name.into(),
        }
    }
}

impl fmt::Display for DynamicState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", self.name)
    }
}

/// A mid-route wildcard parameter.
#[derive(Clone, Debug)]
pub(crate) struct WildcardState {
    pub id: Option<NonZeroUsize>,
    pub name: Box<str>,
}

impl WildcardState {
    pub(crate) fn new(name: &str) -> Self {
        Self {
            id: None,
            name: name.into(),
        }
    }
}

impl fmt::Display for WildcardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<*{}>", self.name)
    }
}

/// An end-of-route catch-all wildcard.
#[derive(Clone, Debug)]
pub(crate) struct EndWildcardState<T> {
    pub name: Box<str>,
    /// Stores data directly to avoid the need for the full `Node` machinery.
    pub data: Data<T>,
}

impl<T> EndWildcardState<T> {
    pub(crate) fn new(name: &str, data: Data<T>) -> Self {
        Self {
            name: name.into(),
            data,
        }
    }
}

impl<T> fmt::Display for EndWildcardState<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<*{}>", self.name)
    }
}
