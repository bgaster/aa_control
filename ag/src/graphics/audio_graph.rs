

//use crate::native::audio_graph;
use crate::native::*;
use crate::style::style::*;

use iced_graphics::{Backend, Primitive, Renderer, Background, defaults};
use iced_graphics::canvas::{path::Arc, Frame, Path, Stroke};
use iced_native::{mouse, Point, Rectangle, Layout, Vector, Element, Color, Size};

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
        dragging: Option<(node::Node, Point, Point)>,
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

        let ag_bounds = layout.bounds(); 

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

                if let Some((dragging, origin, cursor_position)) = dragging {
                    if *id == dragging {
                        dragged_node = Some((i, layout, origin));
                    }
                    primitive
                }
                else {
                    // clip node within audio-graph
                    Primitive::Clip {
                        bounds: ag_bounds,
                        offset: Vector::new(0, 0),
                        content: Box::new(primitive),
                    }
                }
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

            // draw patches
            let (n1, c1) = &content[0];
            let (n2, c2) = &content[1];

            let l: Vec<_> = layout.children().collect();
            
            let l2 = l[0].bounds();
            let l1 = l[1].bounds();

            //println!("{:?} {:?} {:?}", l1, l2, ag_bounds);

            let p = 
                if let Some((index, layout, origin)) = dragged_node {
                    let bounds = layout.bounds();

                    let x = cursor_position.x  - origin.x;
                    let y = cursor_position.y - origin.y;

                    Path::new(|path| {
                        path.move_to(Point::new(l1.x + l1.width, l1.y));
                        path.quadratic_curve_to(
                            Point::new(l1.x + 80.0 + l1.width, l1.y + 90.0), 
                            Point::new(x,  y));
                    })
                }
                else {
                    Path::new(|path| {
                        path.move_to(Point::new(l1.x + l1.width, l1.y));
                        path.quadratic_curve_to(
                            Point::new(l1.x + 80.0 + l1.width, l1.y + 90.0), 
                            Point::new(l2.x,  l2.y));
                    })
                };
            
            let mut frame = Frame::new(Size::new(900.0, 900.0));

            // let e_stroke = Stroke {
            //     width: 6.0,
            //     color: Color::Black,
            //     ..Stroke::default()
            // };
            frame.stroke(&p, Stroke::default().with_width(4.0));

            let bounds = layout.bounds();
            let style = style.active();
            let bg = background(bounds, &style).unwrap();
            primitives.insert(0, bg);
            primitives.push(frame.into_geometry().into_primitive());

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
        title_bar: Option<(&crate::native::title_bar::TitleBar<'_, Message, Self>, Layout<'_>)>,
        ports: Option<(&crate::native::ports::Ports<Self>, Layout<'_>, Layout<'_>)>,
        body: (&Element<'_, Message, Self>, Layout<'_>),
        cursor_position: Point,
    ) -> Self::Output {
        let style = style_sheet.active();
        let (body, body_layout) = body;

        let (body_primitive, body_interaction) =
            body.draw(self, defaults, body_layout, cursor_position, &bounds);

        let background =  background(bounds, &style);
        if let Some((title_bar, title_bar_layout)) = title_bar {
            let show_controls = bounds.contains(cursor_position);
            let is_over_pick_area =
                title_bar.is_over_pick_area(title_bar_layout, cursor_position);

            let (title_bar_primitive, title_bar_interaction) = title_bar.draw(
                self,
                defaults,
                title_bar_layout,
                cursor_position,
                show_controls,
            );

            if let Some((ports, input_bounds, output_bounds)) = ports {
                let (ports_primitives, ports_interaction) = ports.draw(
                    self,
                    defaults,
                    input_bounds,
                    output_bounds,
                    cursor_position,
                );

                (
                    Primitive::Group {
                        primitives: vec![
                            background.unwrap_or(Primitive::None),
                            title_bar_primitive,
                            ports_primitives,
                            body_primitive,
                        ],
                    },
                    if is_over_pick_area {
                        mouse::Interaction::Grab
                    } else if title_bar_interaction > body_interaction {
                        title_bar_interaction
                    } else {
                        body_interaction
                    },
                )
            }
            else {
                (
                    Primitive::Group {
                        primitives: vec![
                            background.unwrap_or(Primitive::None),
                            title_bar_primitive,
                            body_primitive,
                        ],
                    },
                    if is_over_pick_area {
                        mouse::Interaction::Grab
                    } else if title_bar_interaction > body_interaction {
                        title_bar_interaction
                    } else {
                        body_interaction
                    },
                )
            }
        } else {
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

    fn draw_title_bar<Message>(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        style_sheet: &<Self as crate::native::audio_graph::Renderer>::Style,
        content: (&Element<'_, Message, Self>, Layout<'_>),
        controls: Option<(&Element<'_, Message, Self>, Layout<'_>)>,
        cursor_position: Point,
    ) -> Self::Output {
        let style = style_sheet.style();
        let (title_content, title_layout) = content;

        let defaults = Self::Defaults {
            text: defaults::Text {
                //color: style.text_color.unwrap_or(defaults.text.color),
                color: defaults.text.color, // TODO: Fix this up to use AudioWidget Style
            },
        };

        let background = background(bounds, &style);

        let (title_primitive, title_interaction) = title_content.draw(
            self,
            &defaults,
            title_layout,
            cursor_position,
            &bounds,
        );

        if let Some((controls, controls_layout)) = controls {
            let (controls_primitive, controls_interaction) = controls.draw(
                self,
                &defaults,
                controls_layout,
                cursor_position,
                &bounds,
            );

            (
                Primitive::Group {
                    primitives: vec![
                        background.unwrap_or(Primitive::None),
                        title_primitive,
                        controls_primitive,
                    ],
                },
                controls_interaction.max(title_interaction),
            )
        } else {
            (
                if let Some(background) = background {
                    Primitive::Group {
                        primitives: vec![background, title_primitive],
                    }
                } else {
                    title_primitive
                },
                title_interaction,
            )
        }
    }

    fn draw_ports(
        &mut self,
        defaults: &Self::Defaults,
        input_layout: Layout<'_>,
        output_layout: Layout<'_>,
        style_sheet: &<Self as crate::native::audio_graph::Renderer>::Style,
        num: usize,
        cursor_position: Point,
    ) -> Self::Output {
        let mut mouse_interaction = mouse::Interaction::default();
        
        let inputs_primitives = Primitive::Quad {
            bounds: input_layout.bounds(),
            background: Background::Color(Color::from_rgba(
                0xF0 as f32 / 255.0,
                0xF3 as f32 / 255.0,
                0xA5 as f32 / 255.0,
                1.0
            )),
            border_radius: 1.0,
            border_width: 0.5,
            border_color: Color::from_rgba(
                0xF2 as f32 / 255.0,
                0xF3 as f32 / 255.0,
                0xF5 as f32 / 255.0,
                1.0
            ),
        };

        let outputs_primitives = Primitive::Quad {
            bounds: output_layout.bounds(),
            background: Background::Color(Color::from_rgba(
                0xF0 as f32 / 255.0,
                0xF3 as f32 / 255.0,
                0xA5 as f32 / 255.0,
                1.0
            )),
            border_radius: 1.0,
            border_width: 0.5,
            border_color: Color::from_rgba(
                0xF2 as f32 / 255.0,
                0xF3 as f32 / 255.0,
                0xF5 as f32 / 255.0,
                1.0
            ),
        };

        (
            Primitive::Group {
                primitives: vec![inputs_primitives, outputs_primitives],
            },
            mouse_interaction
        )
    }
}
    
    