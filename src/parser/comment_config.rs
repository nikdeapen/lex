/// A line comment configuration.
#[derive(Copy, Clone, Debug)]
pub struct CommentConfig<K> {
    pub kind: K,
    pub(in crate::parser) delimiter_len: usize,
}
