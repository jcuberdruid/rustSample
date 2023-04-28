use crate::ship::Ship;

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Used {shipIndex: u16, hit: bool},
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
    fn isTileEmpty(tile: (u8, u8)) -> bool { 
        // is tile empty 
        type thisTile = grid[[tile.0] tile.1]; 

        match thisTile {
            thisTile::Empty => return true, 
        } 
        return false; 
    }
    fn SetShipToTile(tile: (u8, u8), ship_index: u16) { 
        // yes -> set tile used, index = ship_index, hit = false, ret true 
        match thisTile {
            thisTile::Empty => thisTile::used {shipIndex: ship_index, hit: false}, 
        }
    }

}
