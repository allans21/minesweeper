mod tiles;

fn main() {
    let t: tiles::tile_display = Default::default();
    println!("{}", t.bomb);
    }