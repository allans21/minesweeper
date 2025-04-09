#[derive(Default)]
pub struct tile {
    pub bomb: bool,
    pub revealed: bool,
    pub flag: bool,
}

pub struct tile_display{
    pub bomb: String,
    pub hidden: String,
    pub revealed: String,
    pub flag: String,
}

impl Default for tile_display{
    fn default() -> Self {
        tile_display {
        bomb: String::from("B"),
        hidden: String::from("H"),
        revealed: String::from(" "),
        flag: String::from("F"),
        }
    }
}