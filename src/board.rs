use crate::interface::Interactions;

#[derive(Clone, Default)]
pub struct Tile { // TODO replace flag and revealed with a String/char. this will either be hidden, bomb or the number of bomb neighbors
    pub bomb: bool,
    pub revealed: bool,
    pub flag: bool,
    pub adj_bombs: u8,
}

pub struct Board {
    pub tiles: Vec<Vec<Tile>>,
    pub revealed: u8,
    pub bombs: u8,
    pub x: usize,
    pub y: usize,
}

impl Board {
    pub fn generate(b: u8, x_val: usize, y_val :usize)->Board {
        let mut board = Board {
            bombs: b,
            revealed: 0,
            tiles: vec![vec![Tile::default(); x_val]; y_val],
            x: x_val,
            y: y_val,
        };

        //TODO for now down the middle, make this random
        for i in 0..x_val{
            board.tiles[i][i].bomb = true;
        }

        //find the adjacent bombs
        for x in 0..x_val{
            for y in 0..y_val{
                if board.tiles[x][y].bomb{
                    //increment all neighbors by one
                    if x > 0 && y > 0 {
                        board.tiles[x-1][y-1].adj_bombs = board.tiles[x][y].adj_bombs+1;
                    }
                    if x > 0 {
                        board.tiles[x-1][y].adj_bombs = board.tiles[x][y].adj_bombs+1;
                    }
                    if x > 0 && y < y_val-1 {
                        board.tiles[x-1][y+1].adj_bombs = board.tiles[x][y].adj_bombs+1;
                    }
                    if y > 0 {
                        board.tiles[x][y-1].adj_bombs = board.tiles[x][y].adj_bombs+1;
                    }
                    if y < y_val-1 {
                        board.tiles[x][y+1].adj_bombs = board.tiles[x][y].adj_bombs+1;
                    }
                    if x < x_val-1 && y > 0 {
                        board.tiles[x+1][y-1].adj_bombs = board.tiles[x][y].adj_bombs+1;
                    }
                    if x < x_val-1 {
                        board.tiles[x+1][y].adj_bombs = board.tiles[x][y].adj_bombs+1;
                    }
                    if x < x_val-1 && y < y_val-1 {
                        board.tiles[x+1][y+1].adj_bombs = board.tiles[x][y].adj_bombs+1;
                    }
                }
            }
        }


        return board;
    }

    pub fn update(&mut self, input: &(usize, usize, Interactions)){
        println!("Enter guess in the form Letter:Number:C/F");
        match input.2{
            Interactions::click=>{
                //reveal clicked tile and every tile next to it that is not touching a bomb
                self.tiles[input.0][input.1].revealed = true;
                let mut vec = Vec::new();
                vec.push((input.0, input.1));
                let mut visited: Vec<Vec<bool>> = vec![vec! [false; self.x]; self.y];
                while !(vec.is_empty()){
                    //pop the vec
                    let working = vec.pop();
                    match working{
                        Some(val) =>{
                            visited[val.0][val.1] = true;
                            //check if next to a bomb, if so label how many bombs and do not add neighbors
                        },
                        None => break,
                    }

                }


            }
            Interactions::flag=>{
                self.tiles[input.0][input.1].flag = true;
            }
            Interactions::parse_error=>{
                println!("Error with input, no change made");
            }
        }
    }
}