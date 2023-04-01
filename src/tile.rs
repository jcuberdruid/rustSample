use crate::ship::Ship;

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Hit,
}

impl Default for Tile { //default trait for enum 
   fn default() -> Self {
       Tile::Empty
   } 
}

pub struct Board { 
    grid: [[Tile ;10]; 10],
}

impl Default for Board {
    fn default() -> Self {
        // Create a new Board with default values for its tiles
        let grid = [[Tile::default(); 10]; 10];
        Self { grid }
    }
}
