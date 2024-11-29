use super::{BufferPosition, PieceTable};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Piece {
    pub buffer_index: usize,
    pub start: BufferPosition,
    pub end: BufferPosition,
    pub line_starts: Vec<usize>,
}

impl Piece {
    pub fn new(
        buffer: usize,
        start: BufferPosition,
        end: BufferPosition,
        line_starts: Vec<usize>,
    ) -> Self {
        Self {
            buffer_index: buffer,
            start,
            end,
            line_starts,
        }
    }

    pub fn len(&self) -> usize {
        self.start.remainder - self.end.remainder
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Start: {:?}\nEnd: {:?}\nLength: {}",
            self.start,
            self.end,
            self.len()
        ))
    }
}

pub fn piece_text<'a, 'b>(
    table: &'a PieceTable,
    piece: &'b Piece,
    offset: (usize, usize),
) -> &'a str {
    let buff = &table.buffers[piece.buffer_index];
    let buff_len = buff.text.len();
    let start = buff_len - piece.start.remainder + offset.0;
    let end = buff_len - piece.end.remainder - offset.1;
    buff.slice(start, end)
}
