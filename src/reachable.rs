use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::needle::NeedleCache;
use crate::node::Node;
use crate::state::StaticState;

/// A single reachability condition.
#[derive(Clone, Debug)]
enum Condition {
    /// The remaining path must end with these bytes.
    EndsWith(Box<[u8]>),
    /// The remaining path must contain these bytes.
    Contains { needle: Box<[u8]>, id: usize },
}

impl Condition {
    fn check(&self, needles: &mut NeedleCache, path: &str, offset: usize) -> bool {
        let remaining = &path.as_bytes()[offset..];
        match self {
            Self::EndsWith(suffix) => {
                remaining.len() >= suffix.len()
                    && suffix
                        .iter()
                        .rev()
                        .zip(remaining.iter().rev())
                        .all(|(a, b)| a == b)
            },
            Self::Contains { needle, id } => needles
                .rightmost(*id, needle, path)
                .is_some_and(|position| position >= offset),
        }
    }
}

/// A group of conditions that must all pass.
#[derive(Clone, Debug)]
struct Group {
    conditions: Box<[Condition]>,
}

impl Group {
    fn single(condition: Condition) -> Self {
        Self {
            conditions: Box::new([condition]),
        }
    }

    fn check(&self, needles: &mut NeedleCache, path: &str, offset: usize) -> bool {
        self.conditions
            .iter()
            .all(|condition| condition.check(needles, path, offset))
    }
}

/// Pre-computed reachability conditions for a node.
#[derive(Clone, Debug, Default)]
pub(crate) struct Reachable {
    groups: Box<[Group]>,
}

impl Reachable {
    /// Whether the remaining path could reach a match through this node.
    pub(crate) fn check(&self, needles: &mut NeedleCache, path: &str, offset: usize) -> bool {
        self.groups.is_empty()
            || self
                .groups
                .iter()
                .any(|group| group.check(needles, path, offset))
    }

    /// Computes reachability conditions for a node's subtree.
    pub(crate) fn compute<S, T>(
        node: &Node<S, T>,
        needles: &mut BTreeMap<Box<[u8]>, usize>,
    ) -> Self {
        // Nodes with data or end wildcards are always reachable.
        if node.data.is_some() || node.end_wildcard.is_some() {
            return Self::default();
        }

        let mut groups = Vec::new();
        let mut prefix = Vec::new();

        for child in &node.static_children {
            let inner = Self::walk_static(child, &mut prefix, needles);
            if inner.is_empty() {
                return Self::default();
            }

            groups.extend(inner);
        }

        if groups.is_empty() {
            return Self::default();
        }

        Self {
            groups: groups.into_boxed_slice(),
        }
    }

    /// Walks a static subtree, returning the constraint groups it produces.
    fn walk_static<T>(
        node: &Node<StaticState, T>,
        prefix: &mut Vec<u8>,
        needles: &mut BTreeMap<Box<[u8]>, usize>,
    ) -> Vec<Group> {
        let mut groups = Vec::new();

        let start = prefix.len();
        prefix.extend_from_slice(&node.state.prefix);

        let has_data = node.data.is_some();
        let has_params = node.has_parameters();

        // Top level parameters can't be pruned.
        if has_params && prefix.len() <= 1 {
            prefix.truncate(start);
            return Vec::new();
        }

        if has_params {
            let len = needles.len();
            let id = *needles.entry(prefix.as_slice().into()).or_insert(len);
            let contains = Condition::Contains {
                needle: prefix.as_slice().into(),
                id,
            };

            let any_unconstrained = Self::parameter_groups(node).any(<[Group]>::is_empty);
            if any_unconstrained {
                groups.push(Group::single(contains));
            } else {
                for inner in Self::parameter_groups(node) {
                    for group in inner {
                        let mut deeper = group.conditions.to_vec();

                        let already_has = deeper.iter().any(
                            |condition| matches!(condition, Condition::Contains { id: other, .. } if *other == id),
                        );

                        if !already_has {
                            deeper.insert(0, contains.clone());
                        }

                        groups.push(Group {
                            conditions: deeper.into_boxed_slice(),
                        });
                    }
                }
            }
        }

        // Exact prefix match if this node has data.
        if has_data {
            groups.push(Group::single(Condition::EndsWith(prefix.as_slice().into())));
        }

        for child in &node.static_children {
            let inner = Self::walk_static(child, prefix, needles);
            if inner.is_empty() {
                prefix.truncate(start);
                return Vec::new();
            }

            groups.extend(inner);
        }

        prefix.truncate(start);
        groups
    }

    /// Yields the reachable constraint groups of parameter children.
    fn parameter_groups<S, T>(node: &Node<S, T>) -> impl Iterator<Item = &[Group]> {
        node.dynamic_children
            .iter()
            .map(|child| &*child.reachable.groups)
            .chain(
                node.wildcard_children
                    .iter()
                    .map(|child| &*child.reachable.groups),
            )
            .chain(node.end_wildcard.is_some().then_some(&[] as &[Group]))
    }
}
