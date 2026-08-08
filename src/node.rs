use alloc::borrow::ToOwned as _;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::ToString as _;
use core::fmt;
use core::num::NonZeroUsize;

use crate::bounds::Bounds;
use crate::needle::NeedleCache;
use crate::reachable::Reachable;
use crate::state::{DynamicState, EndWildcardState, StaticState, WildcardState};
use crate::storage::Storage;
use crate::suffixes::Suffixes;

/// Per-search state.
pub(crate) struct SearchContext<'r, 'p> {
    pub needles: NeedleCache,
    pub caps: Storage<usize, 8>,
    pub parameters: Storage<(&'r str, &'p str), 4>,
}

impl SearchContext<'_, '_> {
    pub(crate) const fn new() -> Self {
        Self {
            needles: NeedleCache::new(),
            caps: Storage::new(),
            parameters: Storage::new(),
        }
    }

    /// Caps a boundary scan to exclude everything an earlier visit covered.
    fn cap(&self, node: Option<NonZeroUsize>, offset: usize, max: usize) -> usize {
        let Some(node) = node else {
            return max;
        };

        match self.caps.get(node.get() - 1) {
            Some(&current) => max.min(current.saturating_sub(offset + 1)),
            None => max,
        }
    }

    /// Lowers a node's cap after a visit fails.
    fn lower(&mut self, node: Option<NonZeroUsize>, offset: usize) {
        let Some(node) = node else {
            return;
        };

        if let Some(current) = self.caps.slot(node.get() - 1, usize::MAX) {
            *current = (*current).min(offset + 1);
        }
    }
}

/// Data stored at a leaf node.
#[derive(Clone, Debug)]
pub(crate) struct Data<T> {
    pub data: T,
    pub template: Box<str>,
}

/// Node children search approach.
#[derive(Clone, Debug)]
pub(crate) enum SearchMode {
    /// All children are whole segments.
    Segment,
    /// Children can have inline suffixes.
    Inline,
}

/// An immutable node in the search tree.
#[derive(Clone, Debug)]
pub(crate) struct Node<S, T> {
    pub state: S,
    pub data: Option<Data<T>>,

    pub static_children: Box<[Node<StaticState, T>]>,
    pub static_first_bytes: Box<[u8]>,
    pub dynamic_children: Box<[Node<DynamicState, T>]>,
    pub wildcard_children: Box<[Node<WildcardState, T>]>,
    pub end_wildcard: Option<EndWildcardState<T>>,

    pub bounds: Bounds,
    pub reachable: Reachable,
    pub suffixes: Suffixes,

    pub dynamic_search: SearchMode,
    pub wildcard_search: SearchMode,

    pub parameterized: bool,
}

impl<S, T> Node<S, T> {
    pub(crate) fn search<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
    ) -> Option<&'r Data<T>> {
        self.search_at(ctx, path, 0)
    }

    fn search_at<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
        if offset == path.len() {
            return self.data.as_ref();
        }

        let length = path.len() - offset;
        if length < self.bounds.lower() || length > self.bounds.upper() {
            return None;
        }

        if let Some(result) = self.search_static(ctx, path, offset) {
            return Some(result);
        }

        if !self.parameterized || !path.is_char_boundary(offset) {
            return None;
        }

        let dynamic = match self.dynamic_search {
            SearchMode::Segment => self.search_dynamic_segment(ctx, path, offset),
            SearchMode::Inline => self.search_dynamic_inline(ctx, path, offset),
        };

        if let Some(result) = dynamic {
            return Some(result);
        }

        let wildcard = match self.wildcard_search {
            SearchMode::Segment => self.search_wildcard_segment(ctx, path, offset),
            SearchMode::Inline => self.search_wildcard_inline(ctx, path, offset),
        };

        if let Some(result) = wildcard {
            return Some(result);
        }

        self.search_end_wildcard(ctx, path, offset)
    }

    fn search_static<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
        let (mut child, mut offset) = self.static_candidate(path, offset)?;

        while offset < path.len() {
            let length = path.len() - offset;
            if length < child.bounds.lower() || length > child.bounds.upper() {
                return None;
            }

            if child.parameterized {
                return child.search_at(ctx, path, offset);
            }

            (child, offset) = child.static_candidate(path, offset)?;
        }

        child.data.as_ref()
    }

    fn static_candidate(
        &self,
        path: &str,
        offset: usize,
    ) -> Option<(&Node<StaticState, T>, usize)> {
        let remaining = &path.as_bytes()[offset..];
        let first = remaining.first()?;

        let index = self
            .static_first_bytes
            .iter()
            .position(|byte| byte == first)?;

        let child = self.static_children.get(index)?;
        let prefix = &child.state.prefix;

        if remaining.len() < prefix.len() || prefix.iter().zip(remaining).any(|(a, b)| a != b) {
            return None;
        }

        Some((child, offset + prefix.len()))
    }

    fn search_dynamic_segment<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
        let remaining = &path.as_bytes()[offset..];

        for child in &self.dynamic_children {
            let id = child.state.id;

            let window = (ctx.cap(id, offset, remaining.len()) + 1).min(remaining.len());
            let limit = match memchr::memchr(b'/', &remaining[..window]) {
                Some(limit) if limit > 0 => limit,
                None if window == remaining.len() => remaining.len(),
                Some(_) | None => {
                    ctx.lower(id, offset);
                    continue;
                },
            };

            if remaining.len() - limit < child.bounds.lower() {
                ctx.lower(id, offset);
                continue;
            }

            let boundary = offset + limit;
            ctx.parameters
                .push((&child.state.name, &path[offset..boundary]));

            if let Some(result) = child.search_at(ctx, path, boundary) {
                return Some(result);
            }

            ctx.parameters.pop();
            ctx.lower(id, offset);
        }

        None
    }

    #[inline(never)]
    fn search_dynamic_inline<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
        let remaining = &path.as_bytes()[offset..];

        for child in &self.dynamic_children {
            let id = child.state.id;

            if remaining.len() <= child.bounds.lower() {
                ctx.lower(id, offset);
                continue;
            }

            if !child.reachable.check(&mut ctx.needles, path, offset) {
                ctx.lower(id, offset);
                continue;
            }

            let max = remaining.len() - child.bounds.lower();
            let bound = ctx.cap(id, offset, max);

            let window = (bound + 1).min(remaining.len());
            let limit = memchr::memchr(b'/', &remaining[..window]);
            let cap = limit.unwrap_or(bound);

            // Try boundaries with known suffix.
            for position in child.suffixes.positions(path, offset, cap) {
                let boundary = offset + position;

                ctx.parameters
                    .push((&child.state.name, &path[offset..boundary]));

                if let Some(result) = child.search_at(ctx, path, boundary) {
                    return Some(result);
                }

                ctx.parameters.pop();
            }

            // Try the segment end as a boundary.
            let limit = match limit {
                Some(limit) if limit > 0 => limit,
                None if window == remaining.len() => remaining.len(),
                Some(_) | None => {
                    ctx.lower(id, offset);
                    continue;
                },
            };

            if remaining.len() - limit < child.bounds.lower() {
                ctx.lower(id, offset);
                continue;
            }

            let boundary = offset + limit;
            ctx.parameters
                .push((&child.state.name, &path[offset..boundary]));

            if let Some(result) = child.search_at(ctx, path, boundary) {
                return Some(result);
            }

            ctx.parameters.pop();
            ctx.lower(id, offset);
        }

        None
    }

    #[inline(never)]
    fn search_wildcard_segment<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
        let remaining = &path.as_bytes()[offset..];

        for child in &self.wildcard_children {
            let id = child.state.id;

            if remaining.len() <= child.bounds.lower() {
                ctx.lower(id, offset);
                continue;
            }

            if !child.reachable.check(&mut ctx.needles, path, offset) {
                ctx.lower(id, offset);
                continue;
            }

            let max = remaining.len() - child.bounds.lower();
            let cap = ctx.cap(id, offset, max);
            let upper = (cap + 1).min(remaining.len());

            let initial = memchr::memrchr(b'/', &remaining[..upper]);
            let positions = core::iter::successors(initial, |&position| {
                memchr::memrchr(b'/', &remaining[..position])
            });

            for position in positions.take_while(|&position| position > 0) {
                let after = &remaining[position..];
                if !child.suffixes.accepts(after) {
                    continue;
                }

                let boundary = offset + position;

                ctx.parameters
                    .push((&child.state.name, &path[offset..boundary]));

                if let Some(result) = child.search_at(ctx, path, boundary) {
                    return Some(result);
                }

                ctx.parameters.pop();
            }

            ctx.lower(id, offset);
        }

        None
    }

    #[inline(never)]
    fn search_wildcard_inline<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
        let remaining = &path.as_bytes()[offset..];

        for child in &self.wildcard_children {
            let id = child.state.id;

            if remaining.len() <= child.bounds.lower() {
                ctx.lower(id, offset);
                continue;
            }

            if !child.reachable.check(&mut ctx.needles, path, offset) {
                ctx.lower(id, offset);
                continue;
            }

            let max = remaining.len() - child.bounds.lower();
            let cap = ctx.cap(id, offset, max);

            for position in child.suffixes.positions(path, offset, cap) {
                let boundary = offset + position;

                ctx.parameters
                    .push((&child.state.name, &path[offset..boundary]));

                if let Some(result) = child.search_at(ctx, path, boundary) {
                    return Some(result);
                }

                ctx.parameters.pop();
            }

            ctx.lower(id, offset);
        }

        None
    }

    fn search_end_wildcard<'r, 'p>(
        &'r self,
        ctx: &mut SearchContext<'r, 'p>,
        path: &'p str,
        offset: usize,
    ) -> Option<&'r Data<T>> {
        let child = self.end_wildcard.as_ref()?;
        ctx.parameters.push((&child.name, &path[offset..]));
        Some(&child.data)
    }
}

impl<S: fmt::Display, T> fmt::Display for Node<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn display_node<S: fmt::Display, T>(
            f: &mut fmt::Formatter<'_>,
            node: &Node<S, T>,
            padding: &str,
            is_root: bool,
            is_last: bool,
        ) -> fmt::Result {
            let key = node.state.to_string();
            if !key.is_empty() {
                if is_root {
                    writeln!(f, "{key}")?;
                } else {
                    let branch = if is_last { "╰─" } else { "├─" };
                    writeln!(f, "{padding}{branch} {key}")?;
                }
            }

            let padding = if !is_root && !key.is_empty() {
                if is_last {
                    format!("{padding}   ")
                } else {
                    format!("{padding}│  ")
                }
            } else {
                padding.to_owned()
            };

            let mut count = node.static_children.len()
                + node.dynamic_children.len()
                + node.wildcard_children.len()
                + usize::from(node.end_wildcard.is_some());

            for child in &node.static_children {
                count -= 1;
                display_node(f, child, &padding, key.is_empty(), count == 0)?;
            }

            for child in &node.dynamic_children {
                count -= 1;
                display_node(f, child, &padding, key.is_empty(), count == 0)?;
            }

            for child in &node.wildcard_children {
                count -= 1;
                display_node(f, child, &padding, key.is_empty(), count == 0)?;
            }

            if let Some(child) = &node.end_wildcard {
                let branch = if key.is_empty() { "" } else { "╰─ " };
                writeln!(f, "{padding}{branch}{child}")?;
            }

            Ok(())
        }

        display_node(f, self, "", true, true)
    }
}
