use alloc::vec;
use alloc::vec::Vec;

use crate::compiler::Compiler;
use crate::errors::InsertError;
use crate::node::Data;
use crate::parser::{Part, Template};
use crate::router::Router;
use crate::state::{DynamicState, EndWildcardState, RootState, StaticState, WildcardState};

/// A mutable builder for constructing a [`Router`].
#[derive(Clone)]
pub struct RouterBuilder<T> {
    root: BuilderNode<RootState, T>,
}

impl<T> RouterBuilder<T> {
    /// Creates a new router builder.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            root: BuilderNode::new(RootState::new()),
        }
    }

    /// Inserts a template with associated data into the router.
    ///
    /// # Errors
    ///
    /// When the template has invalid syntax or conflicts with an existing
    /// route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::RouterBuilder;
    ///
    /// let mut builder: RouterBuilder<usize> = RouterBuilder::new();
    /// builder.insert("/hello", 1)?;
    /// # Ok::<_, Box<dyn core::error::Error>>(())
    /// ```
    pub fn insert(&mut self, template: &str, data: T) -> Result<(), InsertError> {
        let mut parsed = Template::new(template)?;

        if let Some(found) = self.root.conflict(&parsed.parts) {
            return Err(InsertError::Conflict {
                existing: found.template.clone().into(),
            });
        }

        self.root.insert(&mut parsed, Data {
            data,
            template: template.into(),
        });

        Ok(())
    }

    /// Consumes the builder and produces an immutable [`Router`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use wayfind::RouterBuilder;
    ///
    /// let mut builder = RouterBuilder::new();
    /// builder.insert("/users/<id>", 1)?;
    /// builder.insert("/posts/<id>", 2)?;
    ///
    /// let router = builder.build();
    /// let search = router.search("/users/123").unwrap();
    /// # Ok::<_, Box<dyn core::error::Error>>(())
    /// ```
    #[must_use]
    pub fn build(self) -> Router<T> {
        Compiler::run(self.root)
    }
}

/// A mutable builder node.
#[derive(Clone, Debug)]
pub(crate) struct BuilderNode<S, T> {
    pub state: S,
    pub data: Option<Data<T>>,

    pub static_children: Vec<BuilderNode<StaticState, T>>,
    pub dynamic_children: Vec<BuilderNode<DynamicState, T>>,
    pub wildcard_children: Vec<BuilderNode<WildcardState, T>>,
    pub end_wildcard: Option<EndWildcardState<T>>,
}

impl<S, T> BuilderNode<S, T> {
    pub(crate) const fn new(state: S) -> Self {
        Self {
            state,
            data: None,

            static_children: Vec::new(),
            dynamic_children: Vec::new(),
            wildcard_children: Vec::new(),
            end_wildcard: None,
        }
    }

    pub(crate) fn has_parameters(&self) -> bool {
        !self.dynamic_children.is_empty()
            || !self.wildcard_children.is_empty()
            || self.end_wildcard.is_some()
    }

    pub(crate) fn is_segment_only(&self) -> bool {
        !self.has_parameters()
            && self
                .static_children
                .iter()
                .all(|child| child.state.prefix.first() == Some(&b'/'))
    }

    pub(crate) fn insert(&mut self, template: &mut Template<'_>, data: Data<T>) {
        let Some(part) = template.parts.pop() else {
            self.data = Some(data);
            return;
        };

        match part {
            Part::Static { prefix } => self.insert_static(template, data, prefix),
            Part::Dynamic { name } => self.insert_dynamic(template, data, name),
            Part::Wildcard { name } if template.parts.is_empty() => {
                self.insert_end_wildcard(data, name);
            },
            Part::Wildcard { name } => self.insert_wildcard(template, data, name),
        }
    }

    fn insert_static(&mut self, template: &mut Template<'_>, data: Data<T>, prefix: &[u8]) {
        let Some(child) = self
            .static_children
            .iter_mut()
            .find(|child| child.state.prefix[0] == prefix[0])
        else {
            let mut new_child = BuilderNode::new(StaticState::new(prefix));
            new_child.insert(template, data);
            self.static_children.push(new_child);

            return;
        };

        let common_prefix = prefix
            .iter()
            .zip(&child.state.prefix)
            .take_while(|&(a, b)| a == b)
            .count();

        if common_prefix >= child.state.prefix.len() {
            if common_prefix >= prefix.len() {
                child.insert(template, data);
            } else {
                child.insert_static(template, data, &prefix[common_prefix..]);
            }

            return;
        }

        let new_child_a = BuilderNode {
            state: StaticState::new(&child.state.prefix[common_prefix..]),
            data: child.data.take(),

            static_children: core::mem::take(&mut child.static_children),
            dynamic_children: core::mem::take(&mut child.dynamic_children),
            wildcard_children: core::mem::take(&mut child.wildcard_children),
            end_wildcard: core::mem::take(&mut child.end_wildcard),
        };

        let new_child_b = BuilderNode::new(StaticState::new(&prefix[common_prefix..]));
        child.state = StaticState::new(&child.state.prefix[..common_prefix]);

        if prefix[common_prefix..].is_empty() {
            child.static_children = vec![new_child_a];
            child.insert(template, data);
        } else {
            child.static_children = vec![new_child_a, new_child_b];
            child.static_children[1].insert(template, data);
        }
    }

    fn insert_dynamic(&mut self, template: &mut Template<'_>, data: Data<T>, name: &str) {
        if let Some(child) = self
            .dynamic_children
            .iter_mut()
            .find(|child| *child.state.name == *name)
        {
            child.insert(template, data);
        } else {
            let mut child = BuilderNode::new(DynamicState::new(name));
            child.insert(template, data);

            self.dynamic_children.push(child);
        }
    }

    fn insert_wildcard(&mut self, template: &mut Template<'_>, data: Data<T>, name: &str) {
        if let Some(child) = self
            .wildcard_children
            .iter_mut()
            .find(|child| *child.state.name == *name)
        {
            child.insert(template, data);
        } else {
            let mut child = BuilderNode::new(WildcardState::new(name));
            child.insert(template, data);

            self.wildcard_children.push(child);
        }
    }

    fn insert_end_wildcard(&mut self, data: Data<T>, name: &str) {
        self.end_wildcard = Some(EndWildcardState::new(name, data));
    }

    pub(crate) fn conflict(&self, parts: &[Part<'_>]) -> Option<&Data<T>> {
        let Some((part, remaining)) = parts.split_last() else {
            return self.data.as_ref();
        };

        match part {
            Part::Static { prefix } => self.conflict_static(remaining, prefix),
            Part::Dynamic { .. } => self.conflict_dynamic(remaining),
            Part::Wildcard { .. } if remaining.is_empty() => self.conflict_end_wildcard(),
            Part::Wildcard { .. } => self.conflict_wildcard(remaining),
        }
    }

    fn conflict_static(&self, parts: &[Part<'_>], prefix: &[u8]) -> Option<&Data<T>> {
        self.static_children
            .iter()
            .filter(|child| {
                prefix.len() >= child.state.prefix.len()
                    && child.state.prefix.iter().zip(prefix).all(|(a, b)| a == b)
            })
            .find_map(|child| {
                let rest = &prefix[child.state.prefix.len()..];
                if rest.is_empty() {
                    child.conflict(parts)
                } else {
                    child.conflict_static(parts, rest)
                }
            })
    }

    fn conflict_dynamic(&self, parts: &[Part<'_>]) -> Option<&Data<T>> {
        self.dynamic_children
            .iter()
            .find_map(|child| child.conflict(parts))
    }

    fn conflict_wildcard(&self, parts: &[Part<'_>]) -> Option<&Data<T>> {
        self.wildcard_children
            .iter()
            .find_map(|child| child.conflict(parts))
    }

    fn conflict_end_wildcard(&self) -> Option<&Data<T>> {
        self.end_wildcard.as_ref().map(|child| &child.data)
    }
}
