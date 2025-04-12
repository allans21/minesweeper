use crate::tiles::*;

pub trait Render {
    fn render_board(&self, board: &Board);
}