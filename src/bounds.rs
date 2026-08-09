use crate::node::Node;

/// Pre-computed path length bounds for pruning during search.
#[derive(Clone, Debug)]
pub(crate) struct Bounds {
    /// Lower bound on remaining path bytes to reach any match.
    lower: u32,

    /// Upper bound on remaining path bytes that could still match.
    upper: u32,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            lower: u32::MAX,
            upper: 0,
        }
    }
}

impl Bounds {
    pub(crate) fn compute<S, T>(node: &Node<S, T>) -> Self {
        let lower = Self::compute_lower(node);
        let lower = u32::try_from(lower).unwrap_or(u32::MAX);

        let upper = Self::compute_upper(node);
        let upper = u32::try_from(upper).unwrap_or(u32::MAX);

        Self { lower, upper }
    }

    pub(crate) fn lower(&self) -> usize {
        usize::try_from(self.lower).unwrap_or(usize::MAX)
    }

    pub(crate) fn upper(&self) -> usize {
        usize::try_from(self.upper).unwrap_or(usize::MAX)
    }

    fn compute_lower<S, T>(node: &Node<S, T>) -> usize {
        // A node with data can match here with 0 remaining bytes.
        if node.data.is_some() {
            return 0;
        }

        // An end-wildcard needs at least 1 byte.
        if node.end_wildcard.is_some() {
            return 1;
        }

        let static_lengths = node.static_children.iter().map(|child| {
            child
                .state
                .prefix
                .len()
                .saturating_add(child.bounds.lower())
        });

        let dynamic_lengths = node
            .dynamic_children
            .iter()
            .map(|child| child.bounds.lower().saturating_add(1));

        let wildcard_lengths = node
            .wildcard_children
            .iter()
            .map(|child| child.bounds.lower().saturating_add(1));

        static_lengths
            .chain(dynamic_lengths)
            .chain(wildcard_lengths)
            .min()
            .unwrap_or(usize::MAX)
    }

    fn compute_upper<S, T>(node: &Node<S, T>) -> usize {
        // Parameters can consume any input.
        if node.parameterized {
            return usize::MAX;
        }

        node.static_children
            .iter()
            .map(|child| {
                child
                    .state
                    .prefix
                    .len()
                    .saturating_add(child.bounds.upper())
            })
            .max()
            .unwrap_or(0)
    }
}
