use alloc::boxed::Box;
use core::num::NonZeroUsize;

use memchr::memmem::FinderRev;

use crate::node::Node;
use crate::state::StaticState;

/// A single pre-computed suffix pattern.
#[derive(Clone, Debug)]
pub(crate) struct Suffix {
    bytes: Box<[u8]>,
    finder: FinderRev<'static>,
}

impl Suffix {
    fn new(suffix: &str) -> Self {
        let bytes: Box<[u8]> = suffix.as_bytes().into();
        let finder = FinderRev::new(&bytes).into_owned();
        Self { bytes, finder }
    }

    /// The greatest starting position not exceeding the bound.
    fn rfind(&self, remaining: &[u8], bound: usize) -> Option<usize> {
        let upper = (bound + self.bytes.len()).min(remaining.len());
        self.finder.rfind(&remaining[..upper])
    }
}

/// Pre-computed suffix patterns for parameter matching.
#[derive(Clone, Default, Debug)]
pub(crate) struct Suffixes {
    edges: Box<[Suffix]>,
    longest: usize,
}

impl Suffixes {
    /// The length of the longest suffix.
    ///
    /// Zero when there are no suffixes.
    pub(crate) const fn longest(&self) -> usize {
        self.longest
    }

    /// Whether the input starts with any suffix's first edge.
    pub(crate) fn accepts(&self, after: &[u8]) -> bool {
        self.edges.iter().any(|edge| {
            after.len() >= edge.bytes.len() && edge.bytes.iter().zip(after).all(|(a, b)| a == b)
        })
    }

    /// Yields candidate boundary positions, walking from right to left.
    pub(crate) fn positions<'a>(
        &'a self,
        remaining: &'a [u8],
        cap: usize,
    ) -> impl Iterator<Item = NonZeroUsize> + 'a {
        let mut limit = cap;

        core::iter::from_fn(move || {
            let position = self
                .edges
                .iter()
                .filter_map(|edge| edge.rfind(remaining, limit))
                .max()?;

            limit = position.saturating_sub(1);
            NonZeroUsize::new(position)
        })
    }

    /// Computes the suffixes from a node's static descendants.
    pub(crate) fn compute<S, T>(node: &Node<S, T>) -> Self {
        let edges = node
            .static_children
            .iter()
            .map(|child| Suffix::new(&child.state.prefix))
            .collect();

        let longest = node
            .static_children
            .iter()
            .filter_map(|child| Self::walk_static(child))
            .max()
            .unwrap_or(0);

        Self { edges, longest }
    }

    /// Walks a static subtree.
    fn walk_static<T>(node: &Node<StaticState, T>) -> Option<usize> {
        let deeper = node
            .static_children
            .iter()
            .filter_map(|child| Self::walk_static(child))
            .max();

        let is_terminal = node.data.is_some() || node.parameterized;
        let here = is_terminal.then_some(0);

        deeper
            .max(here)
            .map(|length| length + node.state.prefix.len())
    }
}
