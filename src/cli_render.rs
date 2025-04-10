use std::usize;
use crate::tiles::*;

pub struct TileDisplay{
    pub bomb: String,
    pub hidden: String,
    pub revealed: String,
    pub flag: String,
}

impl Default for TileDisplay{
    fn default() -> Self {
        TileDisplay {
        bomb: String::from("B"),
        hidden: String::from("H"),
        revealed: String::from(" "),
        flag: String::from("F"),
        }
    }
}


fn print_hor(len: usize){
    for i in 0..len{
        print!(" {}", i);
    }
    println!();
}

fn print_tiles(tiles: &Vec<Tile>){
    let basic: TileDisplay = Default::default();

    for i in 0..tiles.len(){
        print!("|");
        if tiles[i].revealed {
            print!("{}", basic.revealed);
        }
        else if tiles[i].flag {
            print!("{}", basic.flag);
        }
        else if !tiles[i].revealed {
            print!("{}", basic.hidden);
        }
    }
    print!("|");
    println!();
}

pub fn render_board(board: &Board){
    print_hor(board.x);
    for i in 0..board.tiles.len(){
        print_tiles(&board.tiles[i]);
    }
    print_hor(board.x);
}
