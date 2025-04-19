use crate::board;
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
        //TODO prompt size
        self.board = Board::generate(1,8,8);
    }
    fn play_round(&mut self) -> board::BoardState{
        self.render_engine.render_board(&self.board);
        let user_input = self.user_interface.process_input();
        return self.board.update(&user_input);
    }

    pub fn play_game(&mut self){
        loop{
            match self.play_round(){
                board::BoardState::Win=>{
                    self.render_engine.winner();
                    break;
                }
                board::BoardState::Loss=>{
                    self.render_engine.loser();
                    break;
                }
                board::BoardState::Ongoing=>{}
            }
        }
    }
}