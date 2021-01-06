/// The arrangement of a [`AudioGraph`].
///
/// [`AudioGraph`]: crate::audio_graph::AudioGraph
#[derive(Debug, Clone)]
pub enum Configuration<T> {
    /// A [`Node`].
    ///
    /// [`Node`]: crate::audio_graph::Node
    Node(T),
}