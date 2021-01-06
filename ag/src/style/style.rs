use iced_native::{image, Color, Rectangle};
use iced_graphics::{Background, Primitive};
#[derive(Debug, Clone)]
pub enum Style {
    /// modeled after Muses project
    Muses(MusesStyle),
}

#[derive(Debug, Clone)]
pub struct MusesStyle {
    pub fill: Color,
    pub background: Color,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: Color,
}

pub fn background(
    bounds: Rectangle,
    style: &Style) -> Option<Primitive> {
    match style {
        Style::Muses(style) => {
            Some(Primitive::Quad {
                bounds,
                background: Background::Color(style.background),
                border_radius: style.border_radius,
                border_width: style.border_width,
                border_color: style.border_color,
            })
        }
        _ => {
            None
        }
    } 
}

// A set of rules that dictate the style of an [`AudioGraph`].
///
/// [`AudioGraph`]: 
pub trait StyleSheet {
    /// Produces the style of an active [`HSlider`].
    ///
    /// [`AudioGraph`]: ../../native/h_slider/struct.HSlider.html
    fn active(&self) -> Style;

    /// Produces the style of a hovered [`HSlider`].
    ///
    /// [`AudioGraph`]: ../../native/h_slider/struct.HSlider.html
    fn hovered(&self) -> Style;

    /// Produces the style of an [`HSlider`] that is being dragged.
    ///
    /// [`AudioGraph`]: ../../native/h_slider/struct.HSlider.html
    fn dragging(&self) -> Style;

    fn style(&self) -> Style;
}

struct Default;
impl Default {
    const ACTIVE_STYLE: MusesStyle = MusesStyle {
        fill: Color {
            r: 0.26,
            g: 0.26,
            b: 0.26,
            a: 0.75,
        },
        background: Color {
            r: 0.26,
            g: 0.26,
            b: 0.26,
            a: 0.75,
        },
        border_radius: 0.5,
        border_width: 1.0,
        border_color: Color {
            r: 0.26,
            g: 0.26,
            b: 0.26,
            a: 0.75,
        },
    };
}

impl StyleSheet for Default {
    fn active(&self) -> Style {
        Style::Muses(Self::ACTIVE_STYLE)
    }

    fn hovered(&self) -> Style {
        Style::Muses(MusesStyle {
            ..Self::ACTIVE_STYLE
        })
    }

    fn dragging(&self) -> Style {
        Style::Muses(MusesStyle {
            ..Self::ACTIVE_STYLE
        })
    }

    fn style(&self) -> Style {
        Style::Muses(MusesStyle {
            ..Self::ACTIVE_STYLE
        })
    }
}

impl std::default::Default for Box<dyn StyleSheet> {
    fn default() -> Self {
        Box::new(Default)
    }
}

impl<T> From<T> for Box<dyn StyleSheet>
where
    T: 'static + StyleSheet,
{
    fn from(style: T) -> Self {
        Box::new(style)
    }
}