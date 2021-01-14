use iced_native::{
    event, keyboard, layout, mouse, Clipboard, Element, Event, Hasher, Layout,
    Length, Point, Rectangle, Size, Widget, container, overlay, 
};

use super::title_bar::TitleBar;
use super::ports::Ports;

/// The content of a [`Node`].
///
/// [`Node`]: crate::AudioGraph::node
#[allow(missing_debug_implementations)]
pub struct Content<'a, Message, Renderer: super::audio_graph::Renderer> {
    title_bar: Option<TitleBar<'a, Message, Renderer>>,
    ports: Option<Ports<Renderer>>,
    body: Element<'a, Message, Renderer>,
    style: <Renderer as super::audio_graph::Renderer>::Style,
}


impl<'a, Message, Renderer> Content<'a, Message, Renderer>
where
    Renderer: super::audio_graph::Renderer,
{
    /// Creates a new [`Content`] with the provided body.
    pub fn new(
        body: impl Into<Element<'a, Message, Renderer>>) -> Self {
        Self {
            title_bar: None,
            ports: None,
            body: body.into(),
            style: Default::default(),
        }
    }

    /// Sets the [`TitleBar`] of this [`Content`].
    pub fn title_bar(
        mut self,
        title_bar: TitleBar<'a, Message, Renderer>) -> Self {
        self.title_bar = Some(title_bar);
        self
    }

    /// Sets the [`Ports`] of this [`Content`].
    pub fn ports(
        mut self,
        ports: Ports<Renderer>) -> Self {
        self.ports = Some(ports);
        self
    }

    /// Sets the style of the [`Content`].
    pub fn style(mut self, style: impl Into<<Renderer as super::audio_graph::Renderer>::Style>) -> Self {
        self.style = style.into();
        self
    }
}

impl<'a, Message, Renderer> Content<'a, Message, Renderer>
where
    Renderer: super::audio_graph::Renderer,
{
    /// Draws the [`Content`] with the provided [`Renderer`] and [`Layout`].
    ///
    /// [`Renderer`]: crate::audio_graph::Renderer
    pub fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point) -> Renderer::Output {

        if let Some(title_bar) = &self.title_bar {
            let mut children = layout.children();
            let title_bar_layout = children.next().unwrap();
           
            if let Some(ports) = &self.ports {
                let input_layout = children.next().unwrap();
                let output_layout = children.next().unwrap();
                
                let input_bounds = input_layout.bounds();
                let output_bounds = output_layout.bounds();

                let body_layout = children.next().unwrap();

                renderer.draw_node(
                    defaults,
                    layout.bounds(),
                    &self.style,
                    Some((title_bar, title_bar_layout)),
                    Some((ports, input_layout, output_layout)),
                    (&self.body, body_layout),
                    cursor_position)
            }
            else {
                let body_layout = children.next().unwrap();

                renderer.draw_node(
                    defaults,
                    layout.bounds(),
                    &self.style,
                    Some((title_bar, title_bar_layout)),
                    None,
                    (&self.body, body_layout),
                    cursor_position)
            }
        } else {
            if let Some(ports) = &self.ports {
                let mut children = layout.children();
                let input_layout = children.next().unwrap();
                let output_layout = children.next().unwrap();
              
                renderer.draw_node(
                    defaults,
                    layout.bounds(),
                    &self.style,
                    None,
                    Some((ports, input_layout, output_layout)),
                    (&self.body, layout),
                    cursor_position)
            }
            else {
                renderer.draw_node(
                    defaults,
                    layout.bounds(),
                    &self.style,
                    None,
                    None,
                    (&self.body, layout),
                    cursor_position)
            }
        }
    }

    /// Returns whether the [`Content`] with the given [`Layout`] can be picked
    /// at the provided cursor position.
    pub fn can_be_picked_at(
        &self,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> bool {
        if let Some(title_bar) = &self.title_bar {
            let mut children = layout.children();
            let title_bar_layout = children.next().unwrap();

            title_bar.is_over_pick_area(title_bar_layout, cursor_position)
        } else {
            false
        }
    }

    pub(crate) fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        renderer: &Renderer,
        clipboard: Option<&dyn Clipboard>) -> event::Status {
        let mut event_status = event::Status::Ignored;

        let body_layout = if let Some(title_bar) = &mut self.title_bar {
            let mut children = layout.children();

            event_status = title_bar.on_event(
                event.clone(),
                children.next().unwrap(),
                cursor_position,
                messages,
                renderer,
                clipboard,
            );

            if self.ports.is_some() {
                let _input_layout = children.next().unwrap();
                let _output_layout = children.next().unwrap();
            }

            children.next().unwrap()
        } else {
            if self.ports.is_some() {
                let mut children = layout.children();

                let _input_layout = children.next().unwrap();
                let _output_layout = children.next().unwrap();

                children.next().unwrap()
            }
            else {
                layout
            }
        };

        let body_status = self.body.on_event(
            event,
            body_layout,
            cursor_position,
            messages,
            renderer,
            clipboard,
        );

        event_status.merge(body_status)
    }

    pub(crate) fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits) -> layout::Node {
        if let Some(title_bar) = &self.title_bar {
            let max_size = limits.max();

            let max_width = match self.body.width() {
                Length::Units(width) => { 
                    width as f32
                }
                _ => {
                    max_size.width
                }
            };

            let max_height = match self.body.height() {
                Length::Units(height) => { 
                    height as f32
                }
                _ => {
                    max_size.height
                }
            };

            let max_size = Size::new(max_width, max_height); //TODO: this looks wrong max_width twice!

            let title_bar_layout = title_bar
                .layout(renderer, &layout::Limits::new(Size::ZERO, max_size));

            let title_bar_size = title_bar_layout.size();

            if let Some(ports) = &self.ports {
                let mut input_layout = ports.layout_inputs(
                    renderer, 
                    &layout::Limits::new(
                        Size::ZERO,
                        Size::new(
                            max_size.width,
                            max_size.height - title_bar_size.height,
                        ),
                    ));

                let mut output_layout = ports.layout_outputs(
                    renderer, 
                    &layout::Limits::new(
                        Size::ZERO,
                        Size::new(
                            max_size.width,
                            max_size.height - title_bar_size.height,
                        ),
                    ));

                let input_width = input_layout.as_ref().map_or(0.0, |l| l.bounds().width);
                let output_width = output_layout.as_ref().map_or(0.0, |l| l.bounds().width);

                input_layout.as_mut().map(|l| l.move_to(Point::new(0.0, title_bar_size.height)));
                output_layout.as_mut().map(|l| l.move_to(Point::new(max_size.width - output_width, title_bar_size.height)));

                let mut body_layout = self.body.layout(
                    renderer,
                    &layout::Limits::new(
                        Size::ZERO,
                        Size::new(
                            max_size.width - input_width - output_width,
                            max_size.height - title_bar_size.height,
                        ),
                    ),
                );
    
                body_layout.move_to(Point::new(input_width, title_bar_size.height));
    
                if let Some(input_layout) = input_layout {
                    if let Some(output_layout) = output_layout {
                        layout::Node::with_children(
                            max_size,
                            vec![title_bar_layout, input_layout, output_layout, body_layout],
                        )
                    }
                    else {
                        layout::Node::with_children(
                            max_size,
                            vec![title_bar_layout, input_layout, body_layout],
                        )
                    }
                }
                else {
                    if let Some(output_layout) = output_layout {
                        layout::Node::with_children(
                            max_size,
                            vec![title_bar_layout, output_layout, body_layout],
                        )
                    }
                    else {
                        layout::Node::with_children(
                            max_size,
                            vec![title_bar_layout, body_layout],
                        )
                    }
                }
            }
            else {
                let mut body_layout = self.body.layout(
                    renderer,
                    &layout::Limits::new(
                        Size::ZERO,
                        Size::new(
                            max_size.width,
                            max_size.height - title_bar_size.height,
                        ),
                    ),
                );
    
                body_layout.move_to(Point::new(0.0, title_bar_size.height));
    
                layout::Node::with_children(
                    max_size,
                    vec![title_bar_layout, body_layout],
                )
            }
        } else {
            if let Some(ports) = &self.ports {
                let max_size = limits.max();

                let max_width = match self.body.width() {
                    Length::Units(width) => { 
                        width as f32
                    }
                    _ => {
                        max_size.width
                    }
                };
    
                let max_height = match self.body.height() {
                    Length::Units(height) => { 
                        height as f32
                    }
                    _ => {
                        max_size.height
                    }
                };

                let max_size = Size::new(max_width, max_width);

                let input_layout = ports.layout_inputs(renderer, limits);
                let output_layout = ports.layout_outputs(renderer, limits);

                let input_width = input_layout.as_ref().map_or(0.0, |l| l.bounds().width);
                let output_width = output_layout.as_ref().map_or(0.0, |l| l.bounds().width);

                let mut body_layout = self.body.layout(
                    renderer,
                    &layout::Limits::new(
                        Size::ZERO,
                        Size::new(
                            max_size.width - input_width - output_width,
                            max_size.height,
                        ),
                    ),
                );
    
                body_layout.move_to(Point::new(input_width, 0.0));

                if let Some(input_layout) = input_layout {
                    if let Some(output_layout) = output_layout {
                        layout::Node::with_children(
                            max_size,
                            vec![input_layout, output_layout, body_layout],
                        )
                    }
                    else {
                        layout::Node::with_children(
                            max_size,
                            vec![input_layout, body_layout],
                        )
                    }
                }
                else {
                    if let Some(output_layout) = output_layout {
                        layout::Node::with_children(
                            max_size,
                            vec![output_layout, body_layout],
                        )
                    }
                    else {
                        self.body.layout(renderer, limits)
                    }
                }
            }
            else {
                self.body.layout(renderer, limits)
            }
        }
    }

    pub(crate) fn hash_layout(&self, state: &mut Hasher) {
        if let Some(title_bar) = &self.title_bar {
            title_bar.hash_layout(state);
        }

        if let Some(ports) = &self.ports {
            ports.hash_layout(state);
        }

        self.body.hash_layout(state);
    }


    pub(crate) fn overlay(
        &mut self,
        layout: Layout<'_>) -> Option<overlay::Element<'_, Message, Renderer>> {

        let body_layout = if self.title_bar.is_some() {
            let mut children = layout.children();

            // Overlays only allowed in the pane body, for now at least.
            let _title_bar_layout = children.next();

            if let Some(ports) = &self.ports {
                if ports.ports.input_connections() > 0 {
                    let _inputs_layout = children.next();
                }
                if ports.ports.output_connections() > 0 {
                    let _outputs_layout = children.next();
                }
            }

            children.next()?
        } else {
            if let Some(ports) = &self.ports {
                let mut children = layout.children();

                if ports.ports.input_connections() > 0 {
                    let _inputs_layout = children.next();
                }
                if ports.ports.output_connections() > 0 {
                    let _outputs_layout = children.next();
                }

                children.next()?
            }
            else {
                layout
            }
        };

        self.body.overlay(body_layout)
    }
}


impl<'a, T, Message, Renderer> From<T> for Content<'a, Message, Renderer>
where
    T: Into<Element<'a, Message, Renderer>>,
    Renderer: super::audio_graph::Renderer + container::Renderer,
{
    fn from(element: T) -> Self {
        Self::new(element)
    }
}