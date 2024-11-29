use common::buffers::*;
fn main() {
    book_tests();
}

fn hello_world() {
    let pt = PieceTable::from_str("Hello World!");
    println!("Text: {}", pt.text(0, None));
}

fn book_tests() {
    let mut table = PieceTable::from_file("data/goodandevil.txt");

    println!("Getting Line 0:\n{}", table.line(0));
    let line = "Friedrich Nietzsche (1909-1913)\n";
    println!("Inserting line:\n{}\nAt Position: {}", line, 4);
    table.insert(line, 4);
    println!("Getting Line 0:\n{}", table.text(0, Some(35)));
    println!("Lines: {}", table.line_count());
    println!("Removing [0, 4]: ");
    table.delete(0, 4);
    println!("Getting Line 0:\n{}", table.line(0));
}
