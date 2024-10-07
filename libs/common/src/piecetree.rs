use std::{default, sync::Arc};

const AVERAGE_BUFFER_SIZE: usize = 665535;

pub enum UintArray {
    UintArray(Arc<[usize]>),
    Uint32Array(Arc<[u32]>),
    Uint16Array(Arc<[u16]>),
}

pub fn create_usize_array(arr: Vec<usize>) -> UintArray {
    //  Check the last character
    //  If the last character is a 16 bit number create a u16 array.
    //  Else create a u32 array.
    //  Set the first value in the new array to 0.
    todo!()
}

pub struct LineStarts {
    pub line_starts: Vec<usize>, 
    pub cr: usize, 
    pub lf: usize, 
    pub crlf: usize, 
    pub is_basic_ascii: bool
}

impl LineStarts {
    fn new(line_starts: Vec<usize>, cr: usize, lf: usize, crlf: usize, is_basic_ascii: bool) -> Self {
        LineStarts{ line_starts, cr, lf, crlf, is_basic_ascii }
    }
}

pub fn create_line_starts(r: Vec<usize>, str: Box<str>) -> LineStarts {
    //  Iterate over the string and keep count of carriage returns, line feeds, and
    //  the combination.

    //  If a character is not a tab or outside of regular ascii bounds
    //  set is_basic_ascii false.

    //  Create and return a LineStarts struct
    todo!()
}

pub fn create_line_starts_fast(str: Box<str>, readonly: bool) -> UintArray {
    //  Iterate over the characters in the string and keep track of general
    //  returns.
    //  If its readonly return a Uint16Array or Uint32Array.
    //  Otherwise a UintArray.
    todo!()
}

pub enum NodeColor {
    Black = 0,
    Red = 1,
}

impl Default for NodeColor {
    fn default() -> Self {
        NodeColor::Black
    }
}

pub struct NodePosition {
    //  Piece index
    node: TreeNode,
    //  Remainder in currrent piece.
    remainder: usize,
    //  Node start offset in document.
    node_start_offset: usize,
}

pub struct BufferCursor {
    //  The line number in the current buffer.
    line: usize,
    //  Column number in current buffer.
    column: usize,
}

pub struct Piece {
    buffer_index: usize,
    start: BufferCursor,
    end: BufferCursor,
    length: usize,
    line_feed_cnt: usize,
}

impl Piece {
    pub fn new(buffer_index: usize, start: BufferCursor, end: BufferCursor, length: usize, line_feed_cnt: usize) -> Self{
        Self{ buffer_index, start, end, length, line_feed_cnt }
    }
}

pub trait TextSnapshot {
    fn read() -> Option<Box<str>>;
}

pub struct StringBuffer {
    buffer: String,
    line_starts: UintArray,
}

impl StringBuffer {
    pub fn new(buffer: String, line_starts: UintArray) -> Self {
        Self { buffer, line_starts }
    }
}

pub type Link = Option<Box<TreeNode>>;

#[derive(Default)]
pub struct TreeNode {
    parent: Link,
    left: Link,
    right: Link,
    color: NodeColor,

    piece: Option<Piece>,
    size_left: usize,
    line_feeds_left: usize,
}

impl TreeNode {
    pub fn new(piece: Option<Piece>, color: NodeColor) -> Self {
        Self { piece, color, size_left: 0, line_feeds_left: 0, parent: None, left: None, right: None }
    }

    pub fn next() -> Option<Self> {
        todo!()
    }

    pub fn prev() -> Option<Self> {
        todo!()
    }

    pub fn detach() -> () {
        
    }
}

const SENTINEL: TreeNode = TreeNode{ piece: None, parent: None, left: None, right: None, color: NodeColor::Black, size_left: 0, line_feeds_left: 0 };