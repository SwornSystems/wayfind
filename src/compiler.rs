use alloc::boxed::Box;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;
use core::num::NonZeroUsize;

use crate::bounds::Bounds;
use crate::builder::BuilderNode;
use crate::node::{Node, SearchMode};
use crate::reachable::Reachable;
use crate::router::Router;
use crate::state::{DynamicState, RootState, StaticState, WildcardState};
use crate::suffixes::Suffixes;

/// Compiles a builder tree into an optimized tree.
pub(crate) struct Compiler {
    needles: BTreeMap<Box<[u8]>, usize>,
    parameters: usize,
}

impl Compiler {
    pub(crate) fn run<T>(builder: BuilderNode<RootState, T>) -> Router<T> {
        let mut compiler = Self {
            needles: BTreeMap::new(),
            parameters: 0,
        };

        let root = compiler.compile(builder, false);
        Router::new(root)
    }

    fn compile<S, T>(&mut self, builder: BuilderNode<S, T>, revisitable: bool) -> Node<S, T> {
        let mut static_children: Vec<Node<StaticState, T>> = builder
            .static_children
            .into_iter()
            .map(|child| self.compile(child, revisitable))
            .collect();

        let dynamic_inline = !builder
            .dynamic_children
            .iter()
            .all(BuilderNode::is_segment_only);

        let wildcard_inline = !builder
            .wildcard_children
            .iter()
            .all(BuilderNode::is_segment_only);

        let dynamic_revisitable = revisitable || dynamic_inline;
        let wildcard_revisitable = true;

        let mut seen = BTreeSet::new();
        let mut prefix = Vec::new();

        let mut dynamic_children: Vec<Node<DynamicState, T>> = builder
            .dynamic_children
            .into_iter()
            .map(|child| self.compile(child, dynamic_revisitable))
            .collect();

        for child in &mut dynamic_children {
            if revisitable {
                self.parameters += 1;
                child.state.id = NonZeroUsize::new(self.parameters);
            }

            child.suffixes = Suffixes::compute(child, &mut prefix, &mut seen);
            child.reachable = Reachable::compute(child, &mut self.needles);
        }

        let mut wildcard_children: Vec<Node<WildcardState, T>> = builder
            .wildcard_children
            .into_iter()
            .map(|child| self.compile(child, wildcard_revisitable))
            .collect();

        for child in &mut wildcard_children {
            if revisitable {
                self.parameters += 1;
                child.state.id = NonZeroUsize::new(self.parameters);
            }

            child.suffixes = Suffixes::compute(child, &mut prefix, &mut seen);
            child.reachable = Reachable::compute(child, &mut self.needles);
        }

        static_children.sort_by(|a, b| a.state.prefix.cmp(&b.state.prefix));

        dynamic_children.sort_by(|a, b| {
            b.suffixes
                .longest()
                .cmp(&a.suffixes.longest())
                .then_with(|| a.state.name.cmp(&b.state.name))
        });

        wildcard_children.sort_by(|a, b| {
            b.suffixes
                .longest()
                .cmp(&a.suffixes.longest())
                .then_with(|| a.state.name.cmp(&b.state.name))
        });

        let static_first_bytes = static_children
            .iter()
            .filter_map(|child| child.state.prefix.first().copied())
            .collect();

        let dynamic_search = if dynamic_inline {
            SearchMode::Inline
        } else {
            SearchMode::Segment
        };

        let wildcard_search = if wildcard_inline {
            SearchMode::Inline
        } else {
            SearchMode::Segment
        };

        let parameterized = !dynamic_children.is_empty()
            || !wildcard_children.is_empty()
            || builder.end_wildcard.is_some();

        let mut node = Node {
            state: builder.state,
            data: builder.data,

            static_children: static_children.into_boxed_slice(),
            static_first_bytes,
            dynamic_children: dynamic_children.into_boxed_slice(),
            wildcard_children: wildcard_children.into_boxed_slice(),
            end_wildcard: builder.end_wildcard,

            bounds: Bounds::default(),
            reachable: Reachable::default(),
            suffixes: Suffixes::default(),

            dynamic_search,
            wildcard_search,

            parameterized,
        };

        node.bounds = Bounds::compute(&node);
        node
    }
}
