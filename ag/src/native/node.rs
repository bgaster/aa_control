
/// A rectangular region in a [`AudioGraph`] used to display widgets for an audio node.
///
/// [`AudioGraph`]: crate::AudioGraph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node {
    pub(super) id: usize,

}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id
        }
    }
}