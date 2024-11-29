#![allow(unused_assignments)]
#![allow(unused_variables)]
use std::{
    fs::read_to_string,
    ops::{Index, IndexMut},
};

use crate::buffers::line_starts;

use super::{Buffer, BufferPosition, Piece, TextBuffer};

#[derive(Debug)]
pub struct PieceTable {
    pub buffers: Vec<Buffer>,
    pub pieces: Vec<Piece>,
}

impl PieceTable {
    fn piece_text(&self, piece: &Piece, offset: (usize, usize)) -> &str {
        let buff = &self.buffers[piece.buffer_index];
        let buff_len = buff.text.len();
        let start = buff_len - piece.start.remainder + offset.0;
        let end = buff_len - piece.end.remainder - offset.1;
        buff.slice(start, end)
    }

    /// Trims a piece based on a slice.
    fn trim_piece(
        &mut self,
        piece_index: usize,
        start_offset: usize,
        length: usize,
    ) -> usize {
        // println!("\ntrim_piece\n");
        // let b_idx = self.pieces[piece_index].buffer_index;
        // let b_len = self.buffers[b_idx].len();
        let piece: &mut Piece = self.index_mut(piece_index);
        // let buffer = &self.buffers[piece.buffer_index];

        let len: usize; //  Length of the slice to remove from the piece.

        // println!("Length Values:\nParameter: {}", length);

        if length > piece.len() {
            len = piece.len() - start_offset;
        } else {
            len = length;
        }
        // println!("Len: {}\n", len);
        //  Example input Piece = "Hello, World!" : Indexes [0..13]
        //  start_offset = 0
        //  length = 6
        // let b_start = b_len - piece.start.remainder + start_offset;
        // let b_end = b_start + len;
        // println!("Buffer Size: {}", b_len);
        // println!("Buffer Positions:\nStart: {}\nEnd: {}", b_start, b_end);

        let end_offset: usize = piece.len() - len;
        // println!("Offsets: ({}, {})", start_offset, end_offset);
        //	Edge Case Deletions
        //	Whole pieces
        if start_offset == 0 && end_offset == 0 {
            self.pieces.remove(piece_index);
            // println!("\ntrim_piece DONE\n");
            return len;
        }
        //	Start
        //	[0..6] in "Hello, World!" is " World!"
        if start_offset == 0 {
            piece.start.remainder -= len;
        }
        //	End
        //	[8..] in "Hello, World!" is "Hello, "
        else if end_offset == 0 {
            piece.end.remainder += len;
        }
        //	Middle
        //	[2..12] in "Hellow, World!" is "Held!"
        else {
            //	Create a left piece = [0..2]  : "He" in "Hello, World!"
            //  Create a right piece = [12..] : "ld!" in "Hello, World!"
            let (left, mut right) = Self::split_at(&piece, start_offset);
            // println!("Left: {:?}\nRight: {:?}", left, right);
            right.start.remainder -= len;
            // println!(
            //     "Left: {}\nRight: {}",
            //     self.piece_text(&left, (0, 0)),
            //     self.piece_text(&right, (0, 0))
            // );
            self.pieces.remove(piece_index);
            self.pieces.insert(piece_index, right);
            self.pieces.insert(piece_index, left);
        }

        // println!("\ntrim_piece DONE\n");
        len
    }

    /// Returns the length of the total buffer.
    pub fn len(&self) -> usize {
        let mut len = 0;
        for p in &self.pieces {
            len += p.len();
        }
        len
    }

    fn split_at(p: &Piece, pos: usize) -> (Piece, Piece) {
        let mut left = Piece {
            buffer_index: p.buffer_index,
            start: BufferPosition {
                index: p.buffer_index,
                remainder: p.start.remainder,
            },
            end: BufferPosition {
                index: p.buffer_index,
                remainder: p.start.remainder - pos,
            },
            line_starts: vec![],
        };

        let mut right = Piece {
            buffer_index: p.buffer_index,
            start: BufferPosition {
                index: p.buffer_index,
                remainder: p.start.remainder - pos,
            },
            end: BufferPosition {
                index: p.buffer_index,
                remainder: p.end.remainder,
            },
            line_starts: vec![],
        };

        if p.line_starts.is_empty() {
            (left, right)
        } else {
            for i in &p.line_starts {
                if *i <= pos {
                    left.line_starts.push(*i);
                } else {
                    right.line_starts.push(*i);
                }
            }
            (left, right)
        }
    }
}

impl TextBuffer for PieceTable {
    fn from_str(text: &str) -> Self {
        let ls = line_starts(text);
        let start = BufferPosition {
            index: 0,
            remainder: text.len(),
        };
        let end = BufferPosition {
            index: 0,
            remainder: 0,
        };
        let piece = Piece {
            buffer_index: 0,
            start,
            end,
            line_starts: ls,
        };
        let orig = Buffer { text: text.into() };
        Self {
            buffers: vec![
                orig,
                Buffer {
                    text: String::new(),
                },
            ],
            pieces: vec![piece],
        }
    }

    fn from_file(file_path: &str) -> Self {
        let text = read_to_string(file_path).expect("Error reading in file.");
        Self::from_str(&text)
    }

    fn text(&self, start_pos: usize, end_pos: Option<usize>) -> String {
        let mut txt = String::new();
        let end: usize;

        if let Some(pos) = end_pos {
            end = pos;
        } else {
            let mut val = 0;
            for i in &self.pieces {
                val += i.len();
            }
            end = val;
        }

        //  Position in the txt buffer.
        let mut txt_start: usize = 0;
        let mut txt_end: usize = 0;

        for i in 0..self.pieces.len() {
            let p = &self[i];

            txt_start = txt_end;
            txt_end += p.len();

            //  If the piece is not included in the slice.
            if start_pos >= txt_end {
                continue;
            }

            //  The starting slice
            if start_pos >= txt_start {
                let start_offset = start_pos - txt_start;
                let end_offset: usize;
                //  The end of the slice is in the piece.
                if end <= txt_end {
                    end_offset = txt_end - end;
                }
                //  The end is in another slice.
                else {
                    end_offset = 0;
                }
                txt.push_str(&self.piece_text(p, (start_offset, end_offset)));
                if end_offset != 0 {
                    break;
                }
            }
            //  The slice is in the middle
            else if txt_start > start_pos && txt_end <= end {
                txt.push_str(&self.piece_text(p, (0, 0)));
            } else if txt_start > start_pos && end <= txt_end {
                let end_offset: usize;
                end_offset = txt_end - end;
                txt.push_str(&self.piece_text(p, (0, end_offset)));
                break;
            }
        }
        txt
    }

    fn line(&self, line_number: usize) -> String {
        let mut start_feed: usize = 0;
        let mut end_feed: usize = 0;
        let mut start_pos: usize = 0;
        let mut end_pos: usize = 0;
        let mut found_start: bool = false;
        let mut found_end: bool = false;
        let line_cnt = self.line_count();

        if line_number == 0 {
            start_pos = 0;
            found_start = true;
        }
        if line_number == line_cnt {
            end_pos = 0;
            found_end = true;
        }
        let mut offset: usize = 0;
        for i in 0..self.pieces.len() {
            let piece = &self[i];
            if piece.line_starts.is_empty() {
                offset += piece.len();
                continue;
            }
            start_feed = end_feed;
            if !piece.line_starts.is_empty() {
                end_feed += piece.line_starts.len();
            }

            if line_number >= start_feed && line_number <= end_feed {
                let idx = line_number - start_feed;

                if !found_start {
                    start_pos = piece.line_starts[idx - 1];
                    found_start = true;
                    if !found_end {
                        end_pos = piece.line_starts[idx];
                        break;
                    }
                    continue;
                } else {
                    end_pos = piece.line_starts[idx];
                    break;
                }
            }
        }
        let e = if end_pos == 0 {
            None
        } else {
            Some(end_pos - 1)
        };

        self.text(start_pos, e)
    }

    fn lines(&self, start: usize, end: Option<usize>) -> Vec<String> {
        self.text(start, end)
            .split("\n")
            .map(|s| s.to_string())
            .collect()
    }

    fn line_count(&self) -> usize {
        let mut cnt = 0;
        for p in &self.pieces {
            cnt += p.line_starts.len();
        }
        cnt
    }

    fn insert(&mut self, txt: &str, pos: usize) {
        let mut txt_start: usize = 0;
        let mut txt_end: usize = 0;
        let mut piece_idx: usize = 0;
        let mut success = false;
        let mut on_start = false;
        let mut on_boundary = false;

        let txt_len = txt.len();
        let last = self.buffers.len() - 1;

        //  Update the buffer in a limited scope to forcefully drop the mutable
        //  reference to self.
        {
            let buffer = &mut self.buffers[last];
            let mut data = buffer.text.clone();
            data.push_str(txt);
            buffer.text = data;
        }

        let mut found = false;

        for i in 0..self.pieces.len() {
            let p = &mut self[i];
            txt_start = txt_end;
            txt_end += p.len();
            if !found && pos >= txt_start && pos <= txt_end {
                piece_idx = i;
                if pos == txt_start {
                    on_boundary = true;
                    on_start = true;
                } else if pos == txt_end {
                    on_boundary = true;
                }
                found = true;
            }

            // if found && p.buffer_index == last && i != piece_idx {
            //     p.start.remainder += txt_len;
            //     p.end.remainder += txt_len;
            // }
        }

        if pos > txt_end {
            on_boundary = true;
        }

        let buffer = &self.buffers[last];

        if on_boundary {
            if on_start {
                //  Create the BufferPositions and then the piece.
                let start_rem = buffer.len();
                let end_rem = start_rem - txt_len;
                println!("Remainders: ({}, {})", start_rem, end_rem);
                let start = BufferPosition {
                    index: last,
                    remainder: start_rem,
                };
                let end = BufferPosition {
                    index: last,
                    remainder: end_rem,
                };
                let piece = Piece {
                    buffer_index: last,
                    start,
                    end,
                    line_starts: line_starts(txt),
                };
                self.pieces.insert(piece_idx, piece);
                success = true;
            } else {
                //  Create the BufferPositions relative to the end.
                piece_idx = self.pieces.len() - 1;
                let start_rem = txt_len;
                let end_rem = 0;
                let start = BufferPosition {
                    index: last,
                    remainder: start_rem,
                };
                let end = BufferPosition {
                    index: last,
                    remainder: end_rem,
                };
                let piece = Piece {
                    buffer_index: last,
                    start,
                    end,
                    line_starts: line_starts(txt),
                };
                self.pieces.insert(piece_idx + 1, piece);
                success = true;
            }
        } else {
            let orig = &self.pieces[piece_idx];

            let offset = pos;
            let (left, right) = Self::split_at(orig, offset);
            let start_rem = buffer.len();
            let end_rem = 0;
            let start = BufferPosition {
                index: last,
                remainder: start_rem,
            };
            let end = BufferPosition {
                index: last,
                remainder: end_rem,
            };
            let piece = Piece {
                buffer_index: last,
                start,
                end,
                line_starts: line_starts(txt),
            };
            self.pieces.remove(piece_idx);
            self.pieces.insert(piece_idx, right);
            self.pieces.insert(piece_idx, piece);
            self.pieces.insert(piece_idx, left);
        }

        if !success {
            return;
        }

        if on_start {
            let p_idx = 0;
            let mut idx = 0;
            for p in &mut self.pieces {
                if p.buffer_index == last {
                    if idx != p_idx {
                        p.start.remainder += txt_len;
                        p.end.remainder += txt_len;
                        // println!("{} : {:?}", idx, p);
                    }
                }
                idx += 1;
            }
            return;
        }

        // println!("Inserted: \"{}\"", txt);
        let p_idx = piece_idx + 1;
        // println!("Index: {}", pI);
        let mut idx = 0;
        for p in &mut self.pieces {
            if p.buffer_index == last {
                if idx != p_idx {
                    p.start.remainder += txt_len;
                    p.end.remainder += txt_len;
                    // println!("{} : {:?}", idx, p);
                }
            }
            idx += 1;
        }
    }

    fn delete(&mut self, start: usize, length: usize) -> usize {
        let len: usize;

        if length == 0 {
            return 0;
        }

        //  If the length produces an index that is out of bounds then
        //  limit the length.
        if (start + length) > self.len() {
            len = self.len() - start;
        } else {
            len = length;
        }

        let end = start + len; //  End position in the text buffer.

        let mut txt_start = 0; //  Starting position of a piece in the buffer.
        let mut start_so: usize = 0; //  Start of the slice in the piece its found in.
        let mut txt_end = 0; //  End position of the current piece in the buffer.
        let mut piece_end_pos: usize = 0; //  End position of the slice in the piece.

        let mut piece_start: usize = 0; //  Index of the piece containing the start of the slice.
        let mut piece_end: usize = 0; //  Index of the piece containing the end of the slice.

        let mut found_start = false; //  Found the start of the slice in the buffer.
        let mut found_end = false; //  Found the end of the slice in the buffer.

        let piece_cnt = self.pieces.len();

        //  Iterate over the pieces to get the starting and ending positions.
        for i in 0..piece_cnt {
            let p = &self.pieces[i];
            txt_start = txt_end;
            txt_end += p.len();

            if found_start && found_end {
                break;
            }

            if !found_start && start >= txt_start && start < txt_end {
                found_start = true;
                piece_start = i;
                let so = start - txt_start;
                start_so = so;
            }

            if !found_end && end <= txt_end {
                found_end = true;
                piece_end = i;
                let eo = txt_end - end;
                piece_end_pos = p.len() - eo;
            }
        }
        let mut trimmed: usize = 0;
        let mut total: usize = 0;

        //  If the slice is a single piece slice then slice that piece
        //  and return.
        if piece_start == piece_end {
            let p_len = self[piece_start].len();
            trimmed = self.trim_piece(piece_start, start_so, len);
            total += trimmed;
            println!("Trimmed: {}\nLength: {}", trimmed, p_len);
            return total;
        }
        let mut removed_offset = 0;

        //  Iterate over the pieces that need to be deleted.
        //  The iterator stops one piece before the end.
        for i in piece_start..(piece_end) {
            println!(
                "Piece Index: {}\nPiece Length: {}\n",
                i - removed_offset,
                self[i - removed_offset].len()
            );
            // println!("Trimming Piece {}...", i);
            //  The start uses the offset found earlier
            if i == piece_start {
                let p_len = self[i].len();
                trimmed = self.trim_piece(i, start_so, self[i].len());
                total += trimmed;
                //  If a piece is deleted then the index shifts over to the left.
                println!("Trimmed: {}\nLength: {}", trimmed, p_len);
                if start_so == 0 {
                    removed_offset += 1;
                    println!("Removed: {}", removed_offset);
                }
            }
            //  The piece is a middle piece in the slices. This means that these
            //  slices will have a start_offset of 0. The length passed into
            //  the trim_piece function will be the pieces length.
            else {
                let pi = i - removed_offset;
                println!("{:?}", self[pi]);
                trimmed = self.trim_piece(pi, 0, self[pi].len());
                total += trimmed;
                removed_offset += 1;
                println!("Removed: {}", removed_offset);
                println!("Trimmed: {}", trimmed);
            }
            println!("Pieces Remaining: {}", self.pieces.len());
        }
        println!("\nFinished Trimming First Section.");
        let idx = piece_end - removed_offset;
        println!("Piece Ending Pos: {}", piece_end_pos);
        println!("End Piece: {}\nEnd Pos: {}", idx, piece_end_pos);
        trimmed = self.trim_piece(idx, 0, piece_end_pos);
        total += trimmed;

        total
    }
}

impl Index<usize> for PieceTable {
    type Output = Piece;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pieces[index]
    }
}

impl IndexMut<usize> for PieceTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pieces[index]
    }
}
