mod board;
mod cli_render;
mod render;
mod interface;
mod cli_interface;
mod game_master;


fn main() {
    let cli = Box::new(cli_render::CLIRender{});
    let user_interface = Box::new(cli_interface::CLIInterface{});
    let mut GM: game_master::GameMaster = game_master::GameMaster::new(user_interface, cli);
    GM.generate_board();

    GM.play_game();

}