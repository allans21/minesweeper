use crate::interface;
use crate::interface::*;
use std::io;

pub struct CLIInterface;

impl Interface for CLIInterface{
    fn process_input(&self) -> (usize, usize, interface::Interactions){
        println!("Enter guess in the form Letter:Number:C/F");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let process: Vec<&str> =input.split(":").collect();

        let mut opt = interface::Interactions::ParseError;
        let mut x: usize = 0;
        let mut y: usize = 0;

        if process.len() == 3 {
            x = (process[0].as_bytes()[0] - 65) as usize; //TODO add overflow protection
            y = process[1].parse().expect("Failed to parse second arg as int"); //TODO needs more robust check

            println!("{}", process[2]);
            if &process[2][..1] == "C" {
                opt = interface::Interactions::Click;
            }
            else if &process[2][..1] == "F"{
                opt = interface::Interactions::Flag;
            }
        }

        return (x, y, opt);
    }

    fn get_difficulty(&self) -> (usize, usize, u8) {
        println!("Would you like to play Easy (9x9, 10 bombs), Medium(16x16, 40 Bombs) or Hard(16x30, 99 bombs)?");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let s = &input[..input.len()-2]; //cut off whatever the new line char is TODO find this char
        println!("Your input: {}",  s);

        match s {
            "Easy"=>{return (9,9,10);},
            "Medium"=>{return (16,16,40);},
            "Hard"=>{return (16,30,99);},
            "Mega"=>{return (99,99,99);},
            &_=>{
                println!("ERROR! Did not recognize string: {}",  input);
                return (0,0,0);
            }, //TODO make this a proper error handling
        }
    }
}