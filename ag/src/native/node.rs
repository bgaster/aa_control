
use iced_native::{Point};

use std::hash::{Hash, Hasher};

/// A rectangular region in a [`AudioGraph`] used to display widgets for an audio node.
///
/// [`AudioGraph`]: crate::AudioGraph
#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub struct Node {
    pub(super) id: usize,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
        }
    }
}

