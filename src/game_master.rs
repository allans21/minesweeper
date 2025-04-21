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
        let mut init_vals:(usize, usize, u8) = (0,0,0);

        while init_vals.0 ==0{
            init_vals = self.user_interface.get_difficulty(); //TODO, replace this with proper error handling
        }
        self.board = Board::generate(init_vals.0,init_vals.1,init_vals.2,);
    }
    fn play_round(&mut self) -> board::BoardState{
        self.render_engine.render_board(&self.board, false);
        let user_input = self.user_interface.process_input();
        return self.board.update(&user_input);
    }

    pub fn play_game(&mut self){
        loop{
            match self.play_round(){
                board::BoardState::Win=>{
                    self.render_engine.winner();
                    self.render_engine.render_board(&self.board, true);
                    break;
                }
                board::BoardState::Loss=>{
                    self.render_engine.loser();
                    self.render_engine.render_board(&self.board, true);
                    break;
                }
                board::BoardState::Ongoing=>{}
            }
        }
    }
}