use iced_native::{
    event, keyboard, layout, mouse, Clipboard, Element, Event, Hasher, Layout,
    Length, Point, Rectangle, Size, Widget, container, overlay,
};

/// The content of a [`Node`].
///
/// [`Node`]: crate::AudioGraph::node
#[allow(missing_debug_implementations)]
pub struct Content<'a, Message, Renderer: super::audio_graph::Renderer> {
    //title_bar: Option<TitleBar<'a, Message, Renderer>>,
    body: Element<'a, Message, Renderer>,
    style: <Renderer as super::audio_graph::Renderer>::Style,
}


impl<'a, Message, Renderer> Content<'a, Message, Renderer>
where
    Renderer: super::audio_graph::Renderer,
{
    /// Creates a new [`Content`] with the provided body.
    pub fn new(body: impl Into<Element<'a, Message, Renderer>>) -> Self {
        Self {
 //           title_bar: None,
            body: body.into(),
            style: Default::default(),
        }
    }

    /// Sets the [`TitleBar`] of this [`Content`].
    // pub fn title_bar(
    //     mut self,
    //     title_bar: TitleBar<'a, Message, Renderer>,
    // ) -> Self {
    //     self.title_bar = Some(title_bar);
    //     self
    // }

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

        renderer.draw_node(
            defaults,
            layout.bounds(),
            &self.style,
            (&self.body, layout),
            cursor_position,
        )
    }

    /// Returns whether the [`Content`] with the given [`Layout`] can be picked
    /// at the provided cursor position.
    pub fn can_be_picked_at(
        &self,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> bool {
        true
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

        let body_layout = layout;

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
            self.body.layout(renderer, limits)
    }

    pub(crate) fn hash_layout(&self, state: &mut Hasher) {
        self.body.hash_layout(state);
    }


    pub(crate) fn overlay(
        &mut self,
        layout: Layout<'_>) -> Option<overlay::Element<'_, Message, Renderer>> {

        let body_layout = layout;

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