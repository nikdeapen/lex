/// A line comment configuration.
#[derive(Copy, Clone)]
pub(in crate::parser) struct CommentConfig<K> {
    pub(in crate::parser) kind: K,
    pub(in crate::parser) delimiter_len: usize,
}
