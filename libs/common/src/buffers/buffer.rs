#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Buffer {
    /// The `Buffer`'s content.
    pub text: String,
}

impl<'a> Buffer {
    pub fn slice(&'a self, start: usize, end: usize) -> &'a str {
        &self.text[start..end]
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BufferPosition {
    /// The `Buffer` that contains the position.
    pub index: usize,
    /// Length remaining in the buffer after this position.
    pub remainder: usize,
}
