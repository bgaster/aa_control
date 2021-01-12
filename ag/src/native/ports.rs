use iced_native::{
    event, keyboard, layout, mouse, Clipboard, Element, Event, Hasher, Layout,
    Length, Point, Rectangle, Size, Widget, container,
};

#[allow(missing_debug_implementations)]
pub struct Ports<Renderer: super::audio_graph::Renderer> {
    ports: Box<dyn Connectors>,
    padding: u16,
    style: <Renderer as super::audio_graph::Renderer>::Style,
}

impl<Renderer> Ports<Renderer>
where
    Renderer: super::audio_graph::Renderer,
{
    /// Creates a new [`Ports`] 
    pub fn new(ports: Box<dyn Connectors>) -> Self {
        Self {
            ports,
            padding: 0,
            style: Default::default(),
        }
    }

    /// Sets the padding of the [`Ports`].
    pub fn padding(mut self, units: u16) -> Self {
        self.padding = units;
        self
    }

    /// Sets the style of the [`Ports`].
    pub fn style(
        mut self,
        style: impl Into<<Renderer as super::audio_graph::Renderer>::Style>) -> Self {
        self.style = style.into();
        self
    }

    pub(crate) fn hash_layout(&self, hasher: &mut Hasher) {
        use std::hash::Hash;
        self.padding.hash(hasher);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PortType(pub(super) usize);

impl PortType {
    pub const fn new() -> Self {
        Self (0)
    }

    pub const fn fresh(self) -> Self {
        Self(self.0 + 1)
    }
}

pub trait Connectors {
    /// number of input connections
    fn input_connections(&self) -> usize;

    /// number of output connections
    fn output_connections(&self) -> usize;

    /// return the type for a given input port
    fn input_port_type(&self, index: usize) -> Option<PortType>;

    /// return the type for a given output port
    fn output_port_type(&self, index: usize) -> Option<PortType>;
}

struct ZeroPorts;

impl Connectors for ZeroPorts {
    fn input_connections(&self) -> usize {
        0
    }

    fn output_connections(&self) -> usize {
        0
    }

    fn input_port_type(&self, _index: usize) -> Option<PortType> {
        None
    }

    fn output_port_type(&self, _index: usize) -> Option<PortType> {
        None
    }
} 


