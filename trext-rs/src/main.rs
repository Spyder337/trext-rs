use common::piecetable::PieceTable;

fn main() {
    let pt = PieceTable::new("Hello World!");
    println!("{}", pt.get_text(None, None));
}
