use crate::board::*;

pub trait Render {
    fn render_board(&self, board: &Board, reveal: bool);
    fn winner(&self);
    fn loser(&self);
}