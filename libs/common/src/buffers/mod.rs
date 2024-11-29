pub mod buffer;
pub mod piece;
pub mod piece_table;
pub mod piece_tree;

pub use buffer::*;
pub use piece::*;
pub use piece_table::*;

use regex::Regex;

pub trait TextBuffer {
    // Creation
    /// Creates a TextBuffer from a string slice.
    fn from_str(text: &str) -> Self;
    /// Creates a TextBuffer from a file on disk.
    fn from_file(file_path: &str) -> Self;
    // Reading
    /// Read a slice of text from the buffer.
    ///
    /// `start` and `end` are inclusive.
    fn text(&self, start: usize, end: Option<usize>) -> String;
    /// Read a line of text from the buffer.
    ///
    /// If `line_number` is out of bounds then the last line is returned.
    fn line(&self, line_number: usize) -> String;
    /// Read in a slice of the lines in the document.
    fn lines(&self, start: usize, end: Option<usize>) -> Vec<String>;
    /// Returns the total number of new line feeds.
    fn line_count(&self) -> usize;
    // Modification
    /// Insert a string into the `TextBuffer`.
    fn insert(&mut self, txt: &str, pos: usize);
    /// Delete a section of text starting at `start` and ending at `start + length`.
    ///
    /// Returns the size of slice that was deleted.
    fn delete(&mut self, start: usize, length: usize) -> usize;
}

pub fn line_starts(txt: &str) -> Vec<usize> {
    let re = Regex::new(r#"(\n|\r)"#).expect("Error parsing regex pattern.");
    let matches = re.find_iter(txt);
    // let mut found: usize = 0;
    let mut ls: Vec<usize> = Vec::new();
    for m in matches {
        // found += 1;
        let pos = m.start() + 1;
        ls.push(pos);
        // println!("Found: {}", found);
        // println!("Newline Position: {}", pos);
        // print!("Match: '{}'", m.as_str());
    }
    ls
}
