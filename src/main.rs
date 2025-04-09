mod tiles;

fn main() {
    let mut gameboard: tiles::Board = tiles::Board::generate(5,5,5);
    gameboard.revealed = 1;
    println!("{}", gameboard.revealed);
    }