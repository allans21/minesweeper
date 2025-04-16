use crate::board::*;
use crate::render::*;
use crate::interface::*;

pub struct GameMaster{
    user_interface: Box<dyn Interface>,
    render_engine: Box<dyn Render>,
    board: Board,
}

impl GameMaster{
    pub fn new(ui: Box<dyn Interface>, re: Box<dyn Render>) -> GameMaster{
        GameMaster{ user_interface: ui, render_engine: re, board: Board::generate(0,0,0)}
    }

    pub fn generate_board(&mut self){
        //in future prompt interface for board size here, for now we will do 15x15 with 20 bombs
        self.board = Board::generate(15,15,15);
    }
    fn play_round(&mut self){
        self.render_engine.render_board(&self.board);
        let user_input = self.user_interface.process_input();
        self.board.update(&user_input);
    }

    pub fn play_game(&mut self){
        loop{
            self.play_round();
        }
    }
}