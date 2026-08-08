use alloc::boxed::Box;
use alloc::collections::BTreeSet;
use alloc::vec::Vec;

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
    fn new(bytes: Vec<u8>) -> Self {
        let finder = FinderRev::new(&bytes).into_owned();
        Self {
            bytes: bytes.into_boxed_slice(),
            finder,
        }
    }

    /// The greatest starting position not exceeding the bound.
    fn rfind(&self, remaining: &[u8], bound: usize) -> Option<usize> {
        let upper = (bound + self.bytes.len()).min(remaining.len());
        self.finder.rfind(&remaining[..upper])
    }
}

/// Pre-computed suffix patterns for parameter matching, sorted longest first.
#[derive(Clone, Default, Debug)]
pub(crate) struct Suffixes(Box<[Suffix]>);

impl Suffixes {
    /// The length of the longest suffix.
    ///
    /// Zero when there are no suffixes.
    pub(crate) fn longest(&self) -> usize {
        self.0.first().map_or(0, |suffix| suffix.bytes.len())
    }

    /// Whether the input starts with any suffix.
    pub(crate) fn accepts(&self, after: &[u8]) -> bool {
        self.0.iter().any(|suffix| {
            after.len() >= suffix.bytes.len() && suffix.bytes.iter().zip(after).all(|(a, b)| a == b)
        })
    }

    /// Yields candidate boundary positions, walking from right to left.
    pub(crate) fn positions<'a>(
        &'a self,
        path: &'a str,
        offset: usize,
        cap: usize,
    ) -> impl Iterator<Item = usize> + 'a {
        let remaining = &path.as_bytes()[offset..];
        let mut limit = cap;

        core::iter::from_fn(move || {
            let position = self
                .0
                .iter()
                .filter_map(|suffix| suffix.rfind(remaining, limit))
                .max()?;

            limit = position.checked_sub(1)?;
            Some(position)
        })
        .filter(move |&position| path.is_char_boundary(offset + position))
    }

    /// Computes the suffix set from a node's static descendants.
    pub(crate) fn compute<S, T>(
        node: &Node<S, T>,
        prefix: &mut Vec<u8>,
        seen: &mut BTreeSet<Vec<u8>>,
    ) -> Self {
        seen.clear();

        for child in &node.static_children {
            Self::walk_static(child, prefix, seen);
        }

        let mut suffixes: Vec<Suffix> = seen.iter().cloned().map(Suffix::new).collect();
        suffixes.sort_unstable_by(|a, b| {
            b.bytes
                .len()
                .cmp(&a.bytes.len())
                .then_with(|| a.bytes.cmp(&b.bytes))
        });

        Self(suffixes.into_boxed_slice())
    }

    /// Walks a static subtree, recording the accumulated prefix at each node
    /// that can end a route.
    fn walk_static<T>(
        node: &Node<StaticState, T>,
        prefix: &mut Vec<u8>,
        seen: &mut BTreeSet<Vec<u8>>,
    ) {
        let start = prefix.len();
        prefix.extend_from_slice(&node.state.prefix);

        let is_terminal = node.data.is_some() || node.parameterized;
        if is_terminal {
            seen.insert(prefix.clone());
        }

        for child in &node.static_children {
            Self::walk_static(child, prefix, seen);
        }

        prefix.truncate(start);
    }
}
