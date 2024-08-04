use board::Board;


mod cell;
mod board;

fn main() {
    let board = Board::new();

    println!("{}", board);
}
