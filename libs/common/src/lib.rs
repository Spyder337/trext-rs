pub mod piecetable;
pub mod tui;

#[cfg(test)]
mod tests {
    use super::piecetable::PieceTable;

    fn new_test_table() -> PieceTable {
        PieceTable::new("Hello World!")
    }

    fn new_test_table_large() -> PieceTable {
        let mut pt = PieceTable::new("Hello");
        pt.insert(" ", 5);
        pt.insert("World!", 6);
        pt
    }

    fn test_text(pt: &PieceTable, test_str: &str) {
        let text = pt.get_text(None, None);
        assert_eq!(test_str, text);
    }

    #[test]
    fn get_text() {
        let pt = new_test_table_large();
        test_text(&pt, "Hello World!");
    }

    #[test]
    fn boundary_insert() {
        let mut pt = new_test_table();
        pt.insert(" Also Cats!", 13);
        test_text(&pt, "Hello World! Also Cats!");
    }

    #[test]
    fn middle_insert() {
        let mut pt = new_test_table();
        pt.insert(" Brave New", 5);
        test_text(&pt, "Hello Brave New World!");
    }

    #[test]
    fn trim_piece_edge() {
        let mut pt = new_test_table();
        pt.trim_piece(0, 0, 6);
        test_text(&pt, "World!");
    }

    #[test]
    fn trim_piece_middle() {
        let mut pt = new_test_table();
        pt.trim_piece(0, 5, 1);
        test_text(&pt, "HelloWorld!");
        assert_eq!(pt.len(), 2);
    }

    #[test]
    fn delete_boundary_piece() {
        let mut pt = new_test_table_large();
        pt.delete(0, 5);
        test_text(&pt, " World!");
        println!("{}", pt.get_text(None, None));
        pt.delete(1, 6);
        test_text(&pt, " ");
    }

    #[test]
    fn delete_piece_range() {
        let mut pt = new_test_table_large();
        pt.delete(0, 11);
        test_text(&pt, "!");
    }

    #[test]
    fn get_piece_pos() {
        let mut pt = new_test_table_large();
        let pi = pt.get_pos_piece(6).unwrap();
        assert_eq!(pi, 2);
    }
}
