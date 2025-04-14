use std::fmt;

pub enum Interactions{
    click,
    flag,
    parse_error,
}

impl fmt::Display for Interactions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Interactions::click => write!(f, "Click"),
            Interactions::flag => write!(f, "Flag"),
            Interactions::parse_error => write!(f, "Parse Error"),
        }
    }
}

pub trait Interface{
    fn process_input(&self) -> (usize, usize, Interactions);
}