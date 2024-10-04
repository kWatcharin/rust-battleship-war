use rand::Rng;


//const EXAMPLE_BOARD_SIZE: [[u32; 10]; 10] = [
//    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 
//    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 
//    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 
//    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 
//    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 
//    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 
//    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 
//    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 
//    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 
//    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 
// ];

// Size of the board game.
pub const BOARD_SIZE: usize = 10;

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellState {
    Empty,
    Ship,
    Hit,
    Miss
}

#[allow(unused)]
#[derive(Debug)]
pub struct Board {
    grid: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    ships: Vec<(usize, usize)>
}

impl Board {
    #[allow(unused)]
    pub fn new() -> Self {
        // Create new board game.
        Self {
            grid: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            ships: Vec::new()
        }
    }

    #[allow(unused)]
    pub fn can_place_ship(&mut self, row: usize, col: usize, size: usize, direction: bool) -> bool {
        // Check if the ship can be place on the board.
        if direction {
            if col + size > BOARD_SIZE { 
                return false; 
            }

            for i in 0..size {
                if self.grid[row][col + i] != CellState::Empty {
                    return false;
                }
            }

        } else {
            if row + size > BOARD_SIZE {
                return false;
            }

            for i in 0..size {
                if self.grid[row + i][col] != CellState::Empty {
                    return false;
                }
            }
        }
        true
    }

    #[allow(unused)]
    pub fn place_ship(&mut self, size: usize) {
        // Random to place ship on the board.
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

        loop {
            let row: usize = rng.gen_range(0..BOARD_SIZE);
            let col: usize = rng.gen_range(0..BOARD_SIZE);
            let direction: bool = rng.gen_bool(0.5);
            if self.can_place_ship(row, col, size, direction) {
                for i in 0..size {
                    let (r, c) = if direction { (row, col + i) } else { (row + i, col) };
                    self.grid[r][c] = CellState::Ship;
                    self.ships.push((r,c));
                }
            }
            break;
        }
    }

    #[allow(unused)]
    pub fn fire(&mut self, row: usize, col: usize) -> CellState {
        // Fire!!!
        match self.grid[row][col] {
            CellState::Empty => {
                self.grid[row][col] = CellState::Miss;
                CellState::Miss
            },
            CellState::Ship => {
                self.grid[row][col] = CellState::Hit;
                CellState::Hit
            },
            _ => CellState::Miss
        }
    }
    
    #[allow(unused)]
    pub fn display(&self, hide_ships: bool) {
        // Display !!!
        print!(" ");

        for i in 0..BOARD_SIZE {
            print!(" {} ", i)
        }

        println!();

        for (i, row) in self.grid.iter().enumerate() {
            print!(" {:2} ", i);
            for cell in row {
                match cell {
                    CellState::Empty => {
                        print!(" \u{25A1} ");
                    },
                    CellState::Ship => {
                        if hide_ships { 
                            print!(" "); 
                        } else { 
                            print!(" \u{25A0} "); 
                        }
                    },
                    CellState::Hit => {
                        print!("\x1b[31m \u{25CF} \x1b[0m");
                    },
                    CellState::Miss => {
                        print!("\x1b[36m \u{00B7} \x1b[0m");
                    }
                }
            }
            println!();
        }
    }

    #[allow(unused)]
    pub fn game_over() {
        // Game Over!!!
        todo!()
    }
}


#[allow(unused)]
pub fn get_player_input() {
    todo!()
}

#[allow(unused)]
pub fn generate_opponent_move() {
    todo!()
}