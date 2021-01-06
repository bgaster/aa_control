use iced_native::{
    event, keyboard, layout, mouse, Clipboard, Element, Event, Hasher, Layout,
    Length, Point, Rectangle, Size, Widget,
};

use std::collections::HashMap;

/// The state of a [`AudioGraph`].
///
/// It keeps track of the state of each [`Node`] and the position of each
/// of there connections
///
/// The [`State`] needs to own any mutable contents a [`Node`] may need. This is
/// why this struct is generic over the type `T`. Values of this type are
/// provided to the view function of [`AudioGraph::new`] for displaying each
/// [`Node`].
///
/// [`AudioGraph`]: crate::audio_graph::AudioGraph
/// [`AudioGraph::new`]: crate::audio_graph::AudioGraph::new
#[derive(Debug, Clone)]
pub struct State<T> {
    pub(super) nodes: HashMap<super::node::Node, T>,
    pub(super) internal: Internal,
}

impl<T> State<T> {
    /// Creates a new [`State`], initializing the first node with the provided
    /// state.
    ///
    /// Alongside the [`State`], it returns the first [`Node`] identifier.
    pub fn new(first_node_state: T) -> (Self, super::node::Node) {
        (
            Self::with_configuration(super::configuration::Configuration::Node(first_node_state)),
            super::node::Node::new(0),
        )
    }

    /// Creates a new [`State`] with the given [`Configuration`].
    pub fn with_configuration(config: impl Into<super::configuration::Configuration<T>>) -> Self {
        let mut nodes =  HashMap::new();

        let (layout, last_id) =
            Self::distribute_content(&mut nodes, config.into(), 0);

        State {
            nodes,
            internal: Internal {
                layout,
                last_id,
                action: Action::Idle,
            },
        }
    }

    /// Returns the total amount of panes in the [`State`].
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the internal state of the given [`Node`], if it exists.
    pub fn get(&self, node: &super::node::Node) -> Option<&T> {
        self.nodes.get(node)
    }

    /// Returns the internal state of the given [`Node`] with mutability, if it
    /// exists.
    pub fn get_mut(&mut self, node: &super::node::Node) -> Option<&mut T> {
        self.nodes.get_mut(node)
    }

    pub fn insert(
        &mut self,
        state: T) -> Option<super::node::Node> {
        
        self.internal.last_id = self.internal.last_id.checked_add(1)?;
        let id = self.internal.last_id;
        let new_node = super::node::Node::new(id);

        let _ = self.nodes.insert(new_node, state);

        let layout_node = std::mem::replace(&mut self.internal.layout, super::layout_node::LayoutNode::Node(new_node));
        self.internal.layout = super::layout_node::LayoutNode::push(layout_node, new_node);

        Some(new_node)
    }

    fn distribute_content(
        nodes: &mut HashMap<super::node::Node, T>,
        content: super::configuration::Configuration<T>,
        next_id: usize,
    ) -> (super::layout_node::LayoutNode, usize) {
        match content {
            super::configuration::Configuration::Node(state) => {
                let id = super::node::Node::new(next_id);
                let _ = nodes.insert(id, state);

                (super::layout_node::LayoutNode::Node(id), next_id + 1)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Internal {
    layout: super::layout_node::LayoutNode,
    last_id: usize,
    action: Action,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Idle,
    Dragging { node: super::node::Node, origin: Point },
}


impl Internal {
    pub fn picked_node(&self) -> Option<(super::node::Node, Point)> {
        match self.action {
            Action::Dragging { node, origin, .. } => Some((node, origin)),
            _ => None,
        }
    }

    pub fn pick_node(&mut self, node: &super::node::Node, origin: Point) {
        self.action = Action::Dragging {
            node: *node,
            origin,
        };
    }

    pub fn node_regions(
        &self,
        spacing: f32,
        size: Size,
    ) -> HashMap<super::node::Node, Rectangle> {
        self.layout.node_regions(spacing, size)
    }

    pub fn idle(&mut self) {
        self.action = Action::Idle;
    }

    pub fn hash_layout(&self, hasher: &mut Hasher) {
        use std::hash::Hash;

        self.layout.hash(hasher);
    }
}