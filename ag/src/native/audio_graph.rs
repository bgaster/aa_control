
use super::node::Node;
use super::content::Content;

use crate::style::style::{StyleSheet};

use std::fmt::Debug;

use iced_native::{
    event, keyboard, layout, mouse, Clipboard, Element, Event, Hasher, Layout,
    Length, Point, Rectangle, Size, Widget, Vector, overlay, container, row
};

#[allow(missing_debug_implementations)]
pub struct AudioGraph<'a, Message, Renderer: self::Renderer> {
    state: &'a mut super::state::Internal,
    elements: Vec<(Node, Content<'a, Message, Renderer>)>,
    width: Length,
    height: Length,
    spacing: u16,
    on_click: Option<Box<dyn Fn(super::node::Node) -> Message + 'a>>,
    on_drag: Option<Box<dyn Fn(DragEvent) -> Message + 'a>>,
    //on_resize: Option<(u16, Box<dyn Fn(ResizeEvent) -> Message + 'a>)>,
    style_sheet: <Renderer as super::audio_graph::Renderer>::Style,
}

impl<'a, Message, Renderer> AudioGraph<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    /// Creates a [`AudioGraph`] with the given [`State`] and view function.
    ///
    /// The view function will be called to display each [`Pane`] present in the
    /// [`State`].
    pub fn new<T>(
        state: &'a mut super::state::State<T>,
        view: impl Fn(super::node::Node, &'a mut T) -> Content<'a, Message, Renderer>,
    ) -> Self {
        let elements = {
            state
                .nodes
                .iter_mut()
                .map(|(node, node_state)| (*node, view(*node, node_state)))
                .collect()
        };

        Self {
            state: &mut state.internal,
            elements,
            width: Length::Fill,
            height: Length::Fill,
            spacing: 0,
            on_click: None,
            on_drag: None,
            //on_resize: None,
            style_sheet: Default::default(),
        }
    }

    /// Sets the width of the [`AudioGraph`].
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`PaneGrid`].
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the spacing _between_ the nodes of the [`AudioGraph`].
    pub fn spacing(mut self, units: u16) -> Self {
        self.spacing = units;
        self
    }

    /// Sets the message that will be produced when a [`AudioGraph`] of the
    /// [`AudioGraph`] is clicked.
    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(super::node::Node) -> Message,
    {
        self.on_click = Some(Box::new(f));
        self
    }

    /// Enables the drag and drop interactions of the [`AudioGraph`], which will
    /// use the provided function to produce messages.
    pub fn on_drag<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(DragEvent) -> Message,
    {
        self.on_drag = Some(Box::new(f));
        self
    }

    pub fn set_style_sheet(mut self, style_sheet: <Renderer as super::audio_graph::Renderer>::Style) -> Self {
        self.style_sheet = style_sheet;
        self
    }
}

impl<'a, Message, Renderer> AudioGraph<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    fn click_node(
        &mut self,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
    ) {
        let mut clicked_region =
            self.elements.iter().zip(layout.children()).filter(
                |(_, layout)| layout.bounds().contains(cursor_position),
            );

        if let Some(((node, content), layout)) = clicked_region.next() {
            if let Some(on_click) = &self.on_click {
                messages.push(on_click(*node));
            }

            if let Some(on_drag) = &self.on_drag {
                if content.can_be_picked_at(layout, cursor_position) {
                    let node_position = layout.position();

                    let origin = cursor_position
                        - Vector::new(node_position.x, node_position.y);

                    //println!("{:?} {:?} {:?}", node_position, cursor_position, origin);
                    self.state.pick_node(node, origin, cursor_position);

                    messages.push(on_drag(DragEvent::Picked { node: *node }));
                }
            }
        }
    }
}

/// An event produced during a drag and drop interaction of a [`PaneGrid`].
#[derive(Debug, Clone, Copy)]
pub enum DragEvent {
    /// A [`Node`] was picked for dragging.
    Picked {
        /// The picked [`Node`].
        node: super::node::Node,
    },
    /// A [`Node`] was picked and then dropped.
    Dropped {
        // The dropped [`Node`].
        node: super::node::Node,
        diff: Point,
    },
    /// A [`Node`] was picked and then dropped outside of other [`Node`]
    /// boundaries.
    Canceled {
        /// The picked [`Node`].
        node: super::node::Node,
    },
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for AudioGraph<'a, Message, Renderer>
where
    Renderer: self::Renderer + container::Renderer,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);
        let size = limits.resolve(Size::ZERO);

        let regions = self.state.positions();
        //let regions = self.state.node_regions(f32::from(self.spacing), size);


        let children = self
            .elements
            .iter()
            .filter_map(|(node, element)| {
                let mut region = regions.get(node)?;
                let region = Rectangle {
                    x: region.x,
                    y: region.y,
                    width: 300.0,
                    height: 300.0
                };
                let size = Size::new(region.width, region.height);
                //let size = Size::new(300.0,300.0);

                //println!("g {:?} {:?}", node, region);

                let mut node =
                    element.layout(renderer, &layout::Limits::new(size, size));

                node.move_to(Point::new(region.x, region.y));

                Some(node)
            })
            .collect();

        layout::Node::with_children(size, children)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        renderer: &Renderer,
        clipboard: Option<&dyn Clipboard>,
    ) -> event::Status {
        let mut event_status = event::Status::Ignored;

        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    let bounds = layout.bounds();

                    if bounds.contains(cursor_position) {
                        event_status = event::Status::Captured;
                        self.click_node(
                            layout,
                            cursor_position,
                            messages);
                    }
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    if let Some((node, _, _)) = self.state.picked_node() {
                        if let Some(on_drag) = &self.on_drag {
                            // let mut dropped_region = self
                            //     .elements
                            //     .iter()
                            //     .zip(layout.children())
                            //     .filter(|(_, layout)| {
                            //         layout.bounds().contains(cursor_position)
                            //     });
                            if let Some((id, origin, prev_cursor_position)) = self.state.picked_node() {
                                println!("Dropped o: {:?} {:?} {:?} {:?}", layout.bounds(), cursor_position, origin, prev_cursor_position);
                                let diff = cursor_position - Vector::new(prev_cursor_position.x, prev_cursor_position.y);
                                println!("Dropped o': {:?}", origin);
                                let event = DragEvent::Dropped { node, diff };
                                messages.push(on_drag(event));
                            }
                            
                            // match dropped_region.next() {
                            //     Some(((target, _), _)) if node != *target => {
                            //         DragEvent::Dropped {
                            //             node,
                            //             target: *target,
                            //         }
                            //     }
                            //     _ => DragEvent::Canceled { node },
                            // };

                           
                        }

                        self.state.idle();

                        event_status = event::Status::Captured;
                    }
                }
                mouse::Event::CursorMoved { .. } => {
                    // event_status =
                    //     self.trigger_resize(layout, cursor_position, messages);
                }
                _ => {}
            },
            _ => {}
        }

        if self.state.picked_node().is_none() {
            self.elements
                .iter_mut()
                .zip(layout.children())
                .map(|((_, node), layout)| {
                    node.on_event(
                        event.clone(),
                        layout,
                        cursor_position,
                        messages,
                        renderer,
                        clipboard,
                    )
                })
                .fold(event_status, event::Status::merge)
        } else {
            event::Status::Captured
        }
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        _viewport: &Rectangle,
    ) -> Renderer::Output {
        println!("in here");
        self::Renderer::draw(
            renderer,
            defaults,
            &self.elements,
            self.state.picked_node(),
            layout,
            &self.style_sheet,
            cursor_position)
    }

    fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash;
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.width.hash(state);
        self.height.hash(state);
        self.state.hash_layout(state);

        for (_, element) in &self.elements {
            element.hash_layout(state);
        }
    }

    fn overlay(
        &mut self,
        layout: Layout<'_>,
    ) -> Option<overlay::Element<'_, Message, Renderer>> {
        self.elements
            .iter_mut()
            .zip(layout.children())
            .filter_map(|((_, node), layout)| node.overlay(layout))
            .next()
    }
}

pub trait Renderer: iced_native::Renderer + iced_native::container::Renderer + Sized  {
    /// The style supported by this renderer.
    type Style: Default;

    /// Draws an [`AudioGraph`].
    ///
    /// It receives:
    /// - the nodes of the [`AudioGraph`]
    /// - the [`Node`] that is currently being dragged
    /// - the [`Layout`] of the [`AudioGraph`] and its nodes
    /// - the cursor position
    /// [`AudioGraph`]: crate::AudioGraph
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        nodes: &[(Node, Content<'_, Message, Self>)],
        dragging: Option<(Node, Point, Point)>,
        layout: Layout<'_>,
        style: &<Self as super::audio_graph::Renderer>::Style,
        cursor_position: Point,
    ) -> Self::Output;

    // Draws a [`Pane`].
    ///
    /// It receives:
    /// - the [`Content`] of the [`Node`]
    /// - the [`Layout`] of the [`Node`] and its elements
    /// - the cursor position
    /// [`AudioGraph`]: crate::AudioGraph
    fn draw_node<Message>(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        style: &<Self as super::audio_graph::Renderer>::Style,
        title_bar: Option<(&super::title_bar::TitleBar<'_, Message, Self>, Layout<'_>)>,
        ports: Option<(&super::ports::Ports<Self>, Rectangle, Rectangle)>,
        body: (&Element<'_, Message, Self>, Layout<'_>),
        cursor_position: Point,
    ) -> Self::Output;

    /// Draws a [`TitleBar`].
    ///
    /// It receives:
    /// - the bounds, style of the [`TitleBar`]
    /// - the style of the [`TitleBar`]
    /// - the content of the [`TitleBar`] with its layout
    /// - the controls of the [`TitleBar`] with their [`Layout`], if any
    /// - the cursor position
    fn draw_title_bar<Message>(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        style: &<Self as super::audio_graph::Renderer>::Style,
        content: (&Element<'_, Message, Self>, Layout<'_>),
        controls: Option<(&Element<'_, Message, Self>, Layout<'_>)>,
        cursor_position: Point,
    ) -> Self::Output;

    /// Draws a [`InputPorts`].
    ///
    /// It receives:
    /// - the bounds, style of the [`InputPorts`]
    /// - the style of the [`InputPorts`]
    /// - the number of the [`InputPorts`] with its layout
    /// - the cursor position
    fn draw_ports(
        &mut self,
        defaults: &Self::Defaults,
        input_bounds: Rectangle,
        output_bounds: Rectangle,
        style: &<Self as super::audio_graph::Renderer>::Style,
        num: usize,
        cursor_position: Point,
    ) -> Self::Output;
}

impl<'a, Message, Renderer> From<AudioGraph<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer + row::Renderer,
    Message: 'a,
{
    fn from(
        audio_graph: AudioGraph<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(audio_graph)
    }
}