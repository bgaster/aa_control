use super::node::Node;

use iced_native::{
    event, keyboard, layout, mouse, Clipboard, Element, Event, Hasher, Layout,
    Length, Point, Rectangle, Size, Widget, Vector,
};

use std::collections::HashMap;

/// A layout node of a [`AudioGraph`].
///
/// [`AudioGraph`]: crate::audio_graph::AudioGraph
#[derive(Debug, Clone)]
pub enum LayoutNode {
    /// The region of this [`LayoutNode`] is taken by a [`Node`].
    Node(Node),
    Nodes(Vec<Node>)
}

impl LayoutNode {
    /// Returns the rectangular region for each [`Pane`] in the [`Node`] given
    /// the spacing between panes and the total available space.
    pub fn node_regions(
        &self,
        spacing: f32,
        size: Size,
    ) -> HashMap<Node, Rectangle> {
        let mut regions = HashMap::new();

        self.compute_regions(
            spacing,
            &Rectangle {
                x: 0.0,
                y: 0.0,
                width: size.width,
                height: size.height,
            },
            &mut regions,
        );

        regions
    }

    pub fn push(ln: Self, node: Node) -> Self {
        match ln {
            LayoutNode::Node(node_prev) => {
                LayoutNode::Nodes(vec![node_prev, node])
            },
            LayoutNode::Nodes(mut nodes) => {
                nodes.push(node);
                LayoutNode::Nodes(nodes)
            }
        }
    }

    fn node(&self) -> Option<Node> {
        match self {
            LayoutNode::Node(node) => Some(*node),
            LayoutNode::Nodes(nodes) => None
        }
    }

    fn first_node(&self) -> Node {
        match self {
            LayoutNode::Node(node) => *node,
            LayoutNode::Nodes(nodes) => nodes[0]
        }
    }

    fn compute_regions(
        &self,
        spacing: f32,
        current: &Rectangle,
        regions: &mut HashMap<Node, Rectangle>) {
        match self {
            LayoutNode::Node(node) => {
                let _ = regions.insert(*node, *current);
            }
            LayoutNode::Nodes(nodes) => {
                nodes
                .iter()
                .for_each(|node| {
                    let _ = regions.insert(*node, *current);
                });
            }
        }
    }
}

impl std::hash::Hash for LayoutNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            LayoutNode::Node(node) => {
                node.hash(state);
            }
            LayoutNode::Nodes(nodes) => {
                nodes
                .iter()
                .for_each(|node| {
                    node.hash(state);
                });
            }
        }
    }
}