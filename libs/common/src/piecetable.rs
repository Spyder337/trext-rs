#![allow(unused_variables, unused_assignments)]
use std::{
    fs,
    ops::{Index, IndexMut},
};

//	Reference to a position in a buffer.
#[derive(Clone, Copy)]
pub struct Piece {
    is_orig: bool,
    start: usize,
    length: usize,
}

impl Piece {
    fn new(is_orig: bool, start: usize, length: usize) -> Self {
        Self {
            is_orig,
            start,
            length,
        }
    }
}

//	Two Buffers and an array of Pieces
pub struct PieceTable {
    buffers: Vec<String>,
    pieces: Vec<Piece>,
    text_len: usize,
}

impl PieceTable {
    pub fn new(orig_txt: &str) -> Self {
        Self {
            buffers: vec![orig_txt.to_string(), String::new()],
            pieces: vec![Piece::new(true, 0, orig_txt.len())],
            text_len: orig_txt.len(),
        }
    }

    pub fn from_file(file_path: &str) -> Self {
        let orig_txt = fs::read_to_string(file_path).expect("Error reading file.");

        Self {
            buffers: vec![orig_txt.to_string(), String::new()],
            pieces: vec![Piece::new(true, 0, orig_txt.len())],
            text_len: orig_txt.len(),
        }
    }

    pub fn find_by_pos(&self, char_pos: usize) -> Option<&Piece> {
        //	Text buffer positions for the current slice.
        let mut txt_start = 0;
        let mut txt_end = 0;

        //	Iterate over pieces until you print the substring.
        for i in 0..self.len() {
            let ip = &self[i];
            //	Set start to previous end and increment the end
            //	by the piece length.
            txt_start = txt_end;
            txt_end += ip.length;
            if char_pos >= txt_start || char_pos <= txt_end {
                return Some(ip);
            }
        }

        None
    }

    pub fn len(&self) -> usize {
        self.pieces.len()
    }

    fn get_buffer_len(&self, buffer_index: usize) -> usize {
        self.buffers[buffer_index].len()
    }

    fn get_buffer_slice<'a>(&self, buffer: &str, start: usize, end: usize) -> String {
        buffer[start..end].to_string()
    }

    pub fn trim_piece(&mut self, piece_index: usize, start_offset: usize, length: usize) -> usize {
        let piece: &mut Piece = &mut self[piece_index];

        let len: usize; //  Length of the slice to remove from the piece.

        if length > piece.length {
            len = piece.length - start_offset;
        } else {
            len = length;
        }

        //  Example input Piece = "Hello, World!" : Indexes [0..13]
        //  start_offset = 0
        //  length = 6
        
        let r_start = piece.start + start_offset; //  Start pos of slice in the piece
        let p_end = piece.start + piece.length; //  End pos of the piece
        let r_end = r_start + len; //  End pos of slice in the piece
        
        let end_offset = p_end - r_end; //  Length from slice end to piece end

        //	Edge Case Deletions
        //	Whole piece
        if start_offset == 0 && end_offset == 0 {
            self.pieces.remove(piece_index);
            return r_end - r_start;
        }
        //	Start
        //	[0..6] in "Hello, World!" is " World!"
        if start_offset == 0 {
            piece.start = r_end;
            piece.length = end_offset;
        }
        //	End
        //	[8..] in "Hello, World!" is "Hello, "
        else if end_offset == 0 {
            piece.length = start_offset;
        }
        //	Middle
        //	[2..12] in "Hellow, World!" is "Held!"
        else {
            //	Create a left piece = [0..2]  : "He" in "Hello, World!"
            //  Create a right piece = [12..] : "ld!" in "Hello, World!"
            let l_start = piece.start;
            let l_len = start_offset;
            let r_start = r_end;
            let r_len = end_offset;
            let lp = Piece::new(piece.is_orig, l_start, l_len);
            let rp = Piece::new(piece.is_orig, r_start, r_len);
            self.pieces.remove(piece_index);
            self.pieces.insert(piece_index, rp);
            self.pieces.insert(piece_index, lp);
        }

        r_end - r_start
    }

    pub fn insert(&mut self, txt: &str, pos: usize) {
        let start = self.get_buffer_len(1);
        self.buffers[1].push_str(txt);
        let p = Piece::new(false, start, txt.len());

        let mut piece_index = 0;
        //	Whether the pos falls on the start of another piece.
        let mut on_boundary = false;
        let mut on_start = false;

        let mut txt_start = 0;
        let mut txt_end = 0;
        let mut success = false;

        //	Find the position of the piece based on its place
        //	in the Pieces vec. If the position is out of the
        //	bounds of the vec then it will be appended to the
        //	piece vec.
        //	Otherwise the piece that it falls on will be split into
        //	two seperate parts. Then the new piece will be inserted
        //	inbetween the previous ones.
        for i in 0..self.len() {
            let ip = &self[i];
            txt_start = txt_end;
            txt_end += ip.length;
            if pos >= txt_start && pos <= txt_end {
                piece_index = i;
                if pos == txt_start {
                    on_boundary = true;
                    //	vec.insert(item, index) inserts at that
                    //	index and shifts to the right.
                    //	If the txt pos is on the start of this piece
                    //	we want to insert to the left of the current
                    //	piece.
                    on_start = true;
                } else if pos == txt_end {
                    on_boundary = true;
                }
                break;
            }
        }

        if pos > txt_end {
            on_boundary = true;
        }

        //	If the piece should be appended.
        if on_boundary {
            if on_start {
                self.pieces.insert(piece_index, p);
                success = true;
            } else {
                self.pieces.insert(piece_index + 1, p);
                success = true;
            }
        }
        //	Split the piece the position falls on and insert the
        //	new pieces.
        //	Example:
        //	Buffer contains the pos 64, which falls on a piece
        //	that starts at txt_start=32 and ends at txt_end=128.
        //	The buffer would be split [32..64] [64..128].
        //	Modify the original piece to end at 64.
        //	Append the new piece and the other part of the original.
        else {
            let orig = &mut self[piece_index];
            //	Find the remainder in the original piece.
            // 	Remainder = 64 - 32 = 32
            let remainder = pos - txt_start;
            // 	Second piece length = 96 - 32 = 64
            let new_len = orig.length - remainder;
            //	Set the first piece's length to the remainder.
            orig.length = remainder;
            let new_start = orig.start + remainder;
            let new_p = Piece::new(orig.is_orig, new_start, new_len);
            let new_index = piece_index + 1;
            self.pieces.insert(new_index, new_p);
            self.pieces.insert(new_index, p);
            success = true;
        }

        if success {
            self.text_len += p.length;
        }
    }

    pub fn delete(&mut self, start: usize, length: usize) {
        let len: usize; //  Length of the slice to remove.

        if length == 0 {
            return;
        }

        if (start + length) > self.text_len {
            len = self.text_len - start;
        } else {
            len = length;
        }

        let end: usize = start + len; //  End position in the text buffer.
        let mut txt_start = 0; //  Starting position of a piece in the buffer.
        let mut start_so: usize = 0; //  Start of the slice in the piece its found in.
        let mut txt_end = 0; //  End position of the current piece in the buffer.
        let mut piece_end_pos: usize = 0; //  End position of the slice in the piece.

        let mut piece_start: usize = 0; //  Index of the piece containing the start of the slice.
        let mut piece_end: usize = 0; //  Index of the piece containing the end of the slice.

        let mut found_start = false; //  Found the start of the slice in the buffer.
        let mut found_end = false; //  Found the end of the slice in the buffer.

        let piece_cnt = self.len();

        for i in 0..piece_cnt {
            let p = &self.pieces[i];
            txt_start = txt_end;
            txt_end += p.length;

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
                piece_end_pos = p.length - eo;
            }
        }

        //  Cases For Deletion:
        //      - Slice ends in the same piece.
        //      - Slice contains a start and an end.

        let mut trimmed: usize = 0;
        let mut total: usize = 0;

        if piece_start == piece_end {
            trimmed = self.trim_piece(piece_start, start_so, len);
            self.text_len -= trimmed;
            total += trimmed;
        } else {
            let mut removed_offset = 0;
            let mut init_start = false;

            for i in piece_start..piece_end {
                if !init_start && i == piece_start {
                    let len = self[i].length;
                    trimmed = self.trim_piece(i, start_so, len);
                    total += trimmed;
                    self.text_len -= trimmed;
                    if trimmed == len {
                        removed_offset += 1;
                    }
                    init_start = true;
                } else {
                    let pi = i - removed_offset;
                    trimmed = self.trim_piece(pi, 0, len);
                    total += trimmed;
                    self.text_len -= trimmed;
                    removed_offset += 1;
                }
            }

            trimmed = self.trim_piece(piece_end - removed_offset, 0, piece_end_pos);
            total += trimmed;
            self.text_len -= trimmed;
        }
    }

    //	Start and end are absolute positions in the buffer.
    pub fn get_text(&self, s_start: Option<usize>, s_end: Option<usize>) -> String {
        //	Locate the starting piece and append it's sliced
        //	text. Then find the end piece and append it's text.
        let mut ret = String::new();
        //	Initialize optional args.
        let mut start: usize = 0;
        let mut end: usize = 0;
        let mut has_end = false;
        if s_end.is_some() {
            end = s_end.unwrap();
            has_end = true;
        }
        if s_start.is_some() {
            start = s_start.unwrap();
        }

        //	Make sure start and end were initialized properly.
        if has_end && start >= end {
            return ret;
        }

        //	Text buffer positions for the current slice.
        let mut txt_start = 0;
        let mut txt_end = 0;

        //	Iterate over pieces until you print the substring.
        for i in 0..self.len() {
            let ip = &self[i];
            //	Set start to previous end and increment the end
            //	by the piece length.
            txt_start = txt_end;
            txt_end += ip.length;
            let buffer = (!ip.is_orig) as usize;

            //	Cases:
            //	Piece contains start and end
            if start >= txt_start && (has_end && end <= txt_end) {
                let start_offset = start - txt_start;
                let end_offset = txt_end - end;
                ret.push_str(
                    self.get_piece_text(buffer, i, start_offset, end_offset)
                        .as_str(),
                );
                break;
            }
            //	Piece contains start
            else if start >= txt_start {
                let start_offset = start - txt_start;
                ret.push_str(self.get_piece_text(buffer, i, start_offset, 0).as_str());
            }
            //	Piece contains end
            else if has_end && end <= txt_end {
                let end_offset = start - txt_end;
                ret.push_str(self.get_piece_text(buffer, i, 0, end_offset).as_str());
                break;
            }
            //	Piece contains neither
            else {
                ret.push_str(self.get_piece_text(buffer, i, 0, 0).as_str());
            }
        }
        ret
    }

    pub fn get_pos_piece(&self, char_index: usize) -> Option<usize> {
        for n in 0..self.pieces.len() {
            let p = self.pieces.get(n).unwrap();
            let end = p.start + p.length;
            if char_index <= end {
                return Some(n);
            }
        }
        None
    }

    fn get_piece_text(
        &self,
        buffer_index: usize,
        piece_index: usize,
        start_offset: usize,
        end_offset: usize,
    ) -> String {
        let buffer = &self.buffers[buffer_index];
        let piece = self[piece_index];
        let ps = piece.start;
        let pl = piece.length;
        let start = piece.start + start_offset;
        let end = piece.length - end_offset + start;
        self.get_buffer_slice(buffer, start, end)
    }

    fn merge_pieces(&mut self){
        let new_orig = self.get_text(None, None);
        let new_pt = PieceTable::new(&new_orig);
        self.buffers = new_pt.buffers;
        self.pieces = new_pt.pieces;
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
