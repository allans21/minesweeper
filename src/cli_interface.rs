use crate::interface;
use crate::interface::*;
use crate::tiles::*;
use std::io;

pub struct CLIInterface;

impl Interface for CLIInterface{
    fn process_input(&self, board: &Board) -> (usize, usize, interface::Interactions){
        println!("Enter guess in the form Letter:Number:C/F");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        println!("Your input: {}",  input);

        let process: Vec<&str> =input.split(":").collect();

        let mut opt = interface::Interactions::parse_error;
        let mut x: usize = 0;
        let mut y: usize=0;

        if process.len() == 3 {
            x = (process[0].as_bytes()[0] - 65) as usize;
            y = process[1].parse().expect("Failed to parse second arg as int");

            println!("{}", process[2]);
            if &process[2][..1] == "C" {
                opt = interface::Interactions::click;
            }
            else if &process[2][..1] == "F"{
                opt = interface::Interactions::flag;
            }
        }

        return (x, y, opt);
    }
}