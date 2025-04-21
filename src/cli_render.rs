// mod cli_renders;
use std::usize;
use crate::board::*;
use crate::render::*;


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

pub struct CLIRender;

impl Render for CLIRender{
    fn render_board( &self, board: &Board, reveal: bool){
        print_hor(board.x);
        for i in 0..board.tiles.len(){
            print_tiles(&board.tiles[i], reveal, i as u8);
        }
        print_hor(board.x);
    }

    fn winner(&self){
        println!("YOU WIN");
    }

    fn loser(&self){
        println!("YOU DIED");
    }
}

fn print_hor(len: usize){
    print!("  "); // For Vertical spacing
    for i in 0..len{
        print!(" {}", i);
    }
    println!();
}

fn print_tiles(tiles: &Vec<Tile>, reveal: bool, row: u8){
    let basic: TileDisplay = Default::default();
    print!("{} ", (row+65) as char);
    for i in 0..tiles.len(){
        print!("|");
        if tiles[i].revealed || reveal {
            if tiles[i].bomb{
                print!("{}", basic.bomb);
            }
            else {
                if tiles[i].adj_bombs > 0{print!("{}", tiles[i].adj_bombs);}
                else {print!("{}", basic.revealed);}
            }
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

