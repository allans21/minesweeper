use crate::interface::Interactions;
use rand::Rng;

#[derive(Clone, Default)]
pub struct Tile {
    pub bomb: bool,
    pub revealed: bool,
    pub flag: bool,
    pub adj_bombs: u8,
}

pub struct Board {
    pub tiles: Vec<Vec<Tile>>,
    pub revealed_tiles: u16,
    pub bombs: u8,
    pub x: usize,
    pub y: usize,
}

pub enum BoardState{
    Win,
    Loss,
    Ongoing,
}

impl Board {
    pub fn generate( x_val: usize, y_val :usize, b: u8)->Board {
        let mut board = Board {
            bombs: b,
            revealed_tiles: 0,
            tiles: vec![vec![Tile::default(); y_val]; x_val],
            x: x_val,
            y: y_val,
        };

        //place bombs
        let mut bomb_count: u8 =0;
        let mut rng = rand::rng();
        while bomb_count < b{
            let x: usize = rng.random_range(0..x_val);
            let y: usize = rng.random_range(0..y_val);
            if board.tiles[x][y].bomb == false{
                board.tiles[x][y].bomb = true;
                bomb_count = bomb_count + 1;
            }
        }

        //find the adjacent bombs
        for x in 0..x_val{
            for y in 0..y_val{
                if board.tiles[x][y].bomb{
                    //increment all neighbors by one
                    if x > 0 && y > 0 {
                        board.tiles[x-1][y-1].adj_bombs = board.tiles[x-1][y-1].adj_bombs+1;
                    }
                    if x > 0 {
                        board.tiles[x-1][y].adj_bombs = board.tiles[x-1][y].adj_bombs+1;
                    }
                    if x > 0 && y < y_val-1 {
                        board.tiles[x-1][y+1].adj_bombs = board.tiles[x-1][y+1].adj_bombs+1;
                    }
                    if y > 0 {
                        board.tiles[x][y-1].adj_bombs = board.tiles[x][y-1].adj_bombs+1;
                    }
                    if y < y_val-1 {
                        board.tiles[x][y+1].adj_bombs = board.tiles[x][y+1].adj_bombs+1;
                    }
                    if x < x_val-1 && y > 0 {
                        board.tiles[x+1][y-1].adj_bombs = board.tiles[x+1][y-1].adj_bombs+1;
                    }
                    if x < x_val-1 {
                        board.tiles[x+1][y].adj_bombs = board.tiles[x+1][y].adj_bombs+1;
                    }
                    if x < x_val-1 && y < y_val-1 {
                        board.tiles[x+1][y+1].adj_bombs = board.tiles[x+1][y+1].adj_bombs+1;
                    }
                }
            }
        }


        return board;
    }

    pub fn update(&mut self, input: &(usize, usize, Interactions)) -> BoardState{
        let mut bs: BoardState = BoardState::Ongoing;
        match input.2{
            Interactions::Click=>{
                //reveal clicked tile and every tile next to it that is not touching a bomb
                let mut vec = Vec::new();
                let mut visited: Vec<Vec<bool>> = vec![vec![false; self.x]; self.y];

                //Only process a click if there is no flag
                if !self.tiles[input.0][input.1].flag {    
                    vec.push((input.0, input.1));
                    while !(vec.is_empty()){
                        //pop the vec
                        let working = vec.pop();
                        match working{
                            Some(val) =>{
                                self.tiles[val.0][val.1].revealed = true;
                                self.revealed_tiles= self.revealed_tiles + 1; //count for game win
                                // only look at neighbor tiles if there is not an adjacent bomb
                                if self.tiles[val.0][val.1].adj_bombs == 0{
                                    if val.0 > 0 && val.1 > 0 {
                                        if self.tiles[val.0-1][val.1-1].revealed == false && visited[val.0-1][val.1-1] == false {vec.push((val.0-1, val.1-1)); visited[val.0-1][val.1-1] = true;}
                                    }
                                    if val.0 > 0 {
                                        if self.tiles[val.0-1][val.1].revealed == false && visited[val.0-1][val.1] == false {vec.push((val.0-1, val.1)); visited[val.0-1][val.1] = true;}
                                    }
                                    if val.0 > 0 && val.1 < self.y-1 {
                                        if self.tiles[val.0-1][val.1+1].revealed == false && visited[val.0-1][val.1+1] == false {vec.push((val.0-1, val.1+1)); visited[val.0-1][val.1+1] = true;}
                                    }
                                    if val.1 > 0 {
                                        if self.tiles[val.0][val.1-1].revealed == false && visited[val.0][val.1-1] == false {vec.push((val.0, val.1-1)); visited[val.0][val.1-1] = true;}
                                    }
                                    if val.1 < self.y-1 {
                                        if self.tiles[val.0][val.1+1].revealed == false && visited[val.0][val.1+1] == false {vec.push((val.0, val.1+1)); visited[val.0][val.1+1] = true;}
                                    }
                                    if val.0 < self.x-1 && val.1 > 0 {
                                        if self.tiles[val.0+1][val.1-1].revealed == false && visited[val.0+1][val.1-1] == false {vec.push((val.0+1, val.1-1)); visited[val.0+1][val.1-1] = true;}
                                    }
                                    if val.0 < self.x-1 {
                                        if self.tiles[val.0+1][val.1].revealed == false && visited[val.0+1][val.1] == false {vec.push((val.0+1, val.1)); visited[val.0+1][val.1] = true;}
                                    }
                                    if val.0 < self.x-1 && val.1 < self.y-1 {
                                        if self.tiles[val.0+1][val.1+1].revealed == false && visited[val.0+1][val.1+1] == false {vec.push((val.0+1, val.1+1)); visited[val.0+1][val.1+1] = true;}
                                    }
                                }
                            },
                            None => break,
                        }
                    }
                    if self.tiles[input.0][input.1].bomb{bs = BoardState::Loss;}
                    else if (self.bombs as u16 + self.revealed_tiles) as usize == self.x * self.y {bs = BoardState::Win;}
                }
            }
            Interactions::Flag=>{
                if self.tiles[input.0][input.1].flag{self.tiles[input.0][input.1].flag = false;}
                else {self.tiles[input.0][input.1].flag = true;}
            }
            Interactions::ParseError=>{
                println!("Error with input, no change made"); //TODO, this needs to be controlled by the interface, perhaps using Result? 
            }
        }
        return bs;
    }
}