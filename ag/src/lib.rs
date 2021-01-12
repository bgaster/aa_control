
pub mod core;
pub mod graphics;
pub mod native;
pub mod style;

#[doc(no_inline)]
pub use crate::core::*;

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    #[doc(no_inline)]
    pub use crate::graphics::{
        audio_graph
    };

    #[doc(no_inline)]
    pub use {
        audio_graph::AudioGraph
    };
}

#[doc(no_inline)]
pub use platform::*;

pub use native::node::Node;
pub use native::state::State;
pub use native::content::Content;
pub use native::audio_graph::DragEvent;
pub use native::title_bar::*;
pub use native::layout_node::*;
pub use native::ports::*;
pub use style::*;