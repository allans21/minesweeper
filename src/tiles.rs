#[derive(Clone, Default)]
pub struct Tile {
    pub bomb: bool,
    pub revealed: bool,
    pub flag: bool,
}

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

pub struct Board {
    pub tiles: Vec<Vec<Tile>>,
    pub revealed: u8,
    pub bombs: u8,
}

impl Board {
    pub fn generate(b: u8, x: usize, y :usize)->Board {
        Board {
            bombs: b,
            revealed: 0,
            tiles: vec![vec![Tile::default(); x]; y],
        }
    }
}