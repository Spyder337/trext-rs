#[cfg(test)]
use crate::buffers::PieceTable;
use crate::buffers::TextBuffer;

fn new_test_table() -> crate::buffers::PieceTable {
    crate::buffers::PieceTable::from_str("Hello World!")
}

fn new_test_table_split() -> crate::buffers::PieceTable {
    let mut pt = crate::buffers::PieceTable::from_str("Hello");
    pt.insert(" ", 5);
    pt.insert("World!", 6);
    pt
}

fn new_test_table_file() -> crate::buffers::PieceTable {
    let pt = crate::buffers::PieceTable::from_file("../../data/goodandevil.txt");
    pt
}

fn test_text(pt: &crate::buffers::PieceTable, test_str: &str) {
    let text = pt.text(0, None);
    assert_eq!(test_str, text);
}

#[test]
fn get_text() {
    let pt = new_test_table_split();
    test_text(&pt, "Hello World!");
}

#[test]
fn insert_end() {
    let mut pt = new_test_table();
    pt.insert(" Also Cats!", 13);
    test_text(&pt, "Hello World! Also Cats!");
}

#[test]
fn insert_start() {
    let mut pt = new_test_table();
    pt.insert("Yell out ", 0);
    test_text(&pt, "Yell out Hello World!");
}

#[test]
fn insert_middle() {
    let mut pt = new_test_table();
    pt.insert(" Brave New", 5);
    test_text(&pt, "Hello Brave New World!");
}

#[test]
fn delete_start() {
    let mut pt = new_test_table_split();

    pt.delete(0, 5);
    test_text(&pt, " World!");
}

#[test]
fn delete_end() {
    let mut pt = new_test_table();
    pt.delete(6, 6);
    test_text(&pt, "Hello ");
}

#[test]
fn delete_piece_range() {
    let mut pt = new_test_table_split();
    println!("{:?}", pt);
    pt.delete(0, 11);
    assert_eq!(&pt.text(0, None), "!");
}

#[test]
fn delete_zero() {
    let mut pt = new_test_table_split();
    let res = pt.delete(0, 0);
    assert_eq!(res, 0);
}

#[test]
fn delete_middle() {
    let mut pt = new_test_table_split();
    pt.delete(5, 1);
    test_text(&pt, "HelloWorld!");
}

#[test]
fn line_read() {
    let pt = new_test_table_file();
    let line = pt.line(0);
    assert_eq!(
        &line,
        "The following is a reprint of the Helen Zimmern translation from German"
    );
}
