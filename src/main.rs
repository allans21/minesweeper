use interface::Interface;
use render::Render;

mod tiles;
mod cli_render;
mod render;
mod interface;
mod cli_interface;

fn main() {
    let mut gameboard: tiles::Board = tiles::Board::generate(5,5,5);
    gameboard.revealed = 1;
    println!("{}", gameboard.revealed);
    let cli = cli_render::CLIRender{};
    let user_interface = cli_interface::CLIInterface{};
    cli.render_board(&gameboard);

    let o = user_interface.process_input(&gameboard);
    println!("output: {} {} {}", o.0, o.1, o.2);
}