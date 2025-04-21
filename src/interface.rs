use std::fmt;

pub enum Interactions{
    Click,
    Flag,
    ParseError,
}

impl fmt::Display for Interactions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Interactions::Click => write!(f, "Click"),
            Interactions::Flag => write!(f, "Flag"),
            Interactions::ParseError => write!(f, "Parse Error"),
        }
    }
}

pub trait Interface{
    fn process_input(&self) -> (usize, usize, Interactions); //X, Y, Click/Flag
    fn get_difficulty(&self) -> (usize, usize, u8); //X, Y, Bombs
}