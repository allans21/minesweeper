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

//Note that x is traditionally in the Y directiona and y is in the tradiational x direction here
//This is due to logic in the board function being written in a way where this is easier to flip here
impl Render for CLIRender{
    fn render_board( &self, board: &Board, reveal: bool){
        print_hor(board.y);
        for i in 0..board.tiles.len(){
            print_tiles(&board.tiles[i], reveal, i as u8);
        }
        print_hor(board.y);
        println!("Bombs: {}, Revealed: {}, in {} Tiles", board.bombs, board.revealed_tiles, board.y*board.x);
    }

    fn winner(&self){
        println!("YOU WIN");
    }

    fn loser(&self){
        println!("YOU DIED");
    }
}

fn pow(n: usize, p: usize) -> usize{
    let mut num:usize = n;
    if p == 0{
        return 1;
    }
    for _i in 1..p{
        num = num*n; 
    }
    return num;
}

fn print_hor(len: usize){
    let mut loops:usize = 1;
    let mut num:usize = len;
    while num/10 > 0{
        num = num/10;
        loops += 1; 
    }

    for l in 0..loops{
        print!("  ");
        for i in 0..len{
            let p = pow(10, loops-l-1);
            let mut prnt = i/p;
            prnt = prnt%(p*10);
            
            print!(" {}", prnt);
        }
        println!();
    }
}

fn print_tiles(tiles: &Vec<Tile>, reveal: bool, row: u8){ //TODO handle numbers greater than 9 for spacing, print vertically
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

