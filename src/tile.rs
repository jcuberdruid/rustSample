use crate::ship::Ship;

#[derive(Clone, Copy)]
pub enum Tile {
    Empty,
    Used {shipIndex: u16, hit: bool},
}

impl Default for Tile { //default trait for enum 
   fn default() -> Self {
       Tile::Empty
   } 
}

pub struct Board { 
    pub grid: Vec<Tile>, 
}

impl Default for Board {
    fn default() -> Self {
        // Create a new Board with default values for its tiles
        let mut grid = vec![Tile::default(); 100];
        //let mut grid = Tile::default(); 100];
        Self { grid }
    }
}

impl Board { 
    pub fn isTileEmpty(self, x: u32, y: u32) -> bool { 
        // yes -> set tile used, index = ship_index, hit = false, ret true 
        let thisTile: Tile = self.grid[(x*y) as usize];
        
        if let Tile::Empty = thisTile {
            println!("Tile is empty");
            return true; 
        }
        return false; 
    }
    pub fn setTile(mut self, x: u32, y: u32, sI: u16) { 
        let thisTile: Tile = Tile::Used{shipIndex: sI, hit: false};
        self.grid[(x*y) as usize] = thisTile; 
    }
}
