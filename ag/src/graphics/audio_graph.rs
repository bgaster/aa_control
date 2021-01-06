

//use crate::native::audio_graph;
use crate::native::*;
use crate::style::style::*;

use iced_graphics::{Backend, Primitive, Renderer, Background};
use iced_native::{mouse, Point, Rectangle, Layout, Vector, Element, Color};

pub type AudioGraph<'a, Message, Backend> = 
    audio_graph::AudioGraph<'a, Message, Renderer<Backend>>;

impl<B> audio_graph::Renderer for Renderer<B>
where
    B: Backend,
{
    type Style = Box<dyn StyleSheet>;

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        content: &[(node::Node, content::Content<'_, Message, Self>)],
        dragging: Option<(node::Node, Point)>,
        layout: Layout<'_>,
        style: &<Self as crate::native::audio_graph::Renderer>::Style,
        cursor_position: Point,
    ) -> Self::Output {
        let node_cursor_position = if dragging.is_some() {
            // TODO: Remove once cursor availability is encoded in the type
            // system
            Point::new(-1.0, -1.0)
        } else {
            cursor_position
        };

        let mut mouse_interaction = mouse::Interaction::default();
        let mut dragged_node = None;

        let mut nodes: Vec<_> = content
            .iter()
            .zip(layout.children())
            .enumerate()
            .map(|(i, ((id, node), layout))| {
                let (primitive, new_mouse_interaction) =
                    node.draw(self, defaults, layout, node_cursor_position);

                if new_mouse_interaction > mouse_interaction {
                    mouse_interaction = new_mouse_interaction;
                }

                if let Some((dragging, origin)) = dragging {
                    if *id == dragging {
                        dragged_node = Some((i, layout, origin));
                    }
                }

                primitive
            })
            .collect();

        let mut primitives = 
            if let Some((index, layout, origin)) = dragged_node {
                let node = nodes.remove(index);
                let bounds = layout.bounds();

                // TODO: Fix once proper layering is implemented.
                // This is a pretty hacky way to achieve layering.
                let clip = Primitive::Clip {
                    bounds: Rectangle {
                        x: cursor_position.x - origin.x,
                        y: cursor_position.y - origin.y,
                        width: bounds.width + 0.5,
                        height: bounds.height + 0.5,
                    },
                    offset: Vector::new(0, 0),
                    content: Box::new(Primitive::Translate {
                        translation: Vector::new(
                            cursor_position.x - bounds.x - origin.x,
                            cursor_position.y - bounds.y - origin.y,
                        ),
                        content: Box::new(node),
                    }),
                };

                nodes.push(clip);

                nodes
            } else {
                nodes
            };

            let bounds = layout.bounds();
            let style = style.active();
            let bg = background(bounds, &style).unwrap();
            primitives.insert(0, bg);

        (
            Primitive::Group { primitives },
            if dragging.is_some() {
                mouse::Interaction::Grabbing
            } else {
                mouse_interaction
            },
        )
    }

    fn draw_node<Message>(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        style_sheet: &<Self as crate::native::audio_graph::Renderer>::Style,
        body: (&Element<'_, Message, Self>, Layout<'_>),
        cursor_position: Point,
    ) -> Self::Output {
        let style = style_sheet.active();
        let (body, body_layout) = body;

        let (body_primitive, body_interaction) =
            body.draw(self, defaults, body_layout, cursor_position, &bounds);

        let background =  background(bounds, &style);
        
        (
            if let Some(background) = background {
                Primitive::Group {
                    primitives: vec![background, body_primitive],
                }
            } else {
                body_primitive
            },
            body_interaction,
        )
    }
}
    
    