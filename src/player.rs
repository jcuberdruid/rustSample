use crate::ship::Ship;
use crate::ship::ShipType;
use crate::tile::Board;

pub struct Player {
    pub ships: Vec<Ship>,
    pub board: Board,
}

impl Default for Player {
    fn default() -> Player { 
        // pub fn setup(&mut self) {
        let board = Board::default();
        let mut ships: Vec<Ship> = Vec::new(); 

        ships.push(Ship::new(ShipType::Carrier));
        ships.push(Ship::new(ShipType::Battleship));
        ships.push(Ship::new(ShipType::Cruiser));
        ships.push(Ship::new(ShipType::Cruiser));
        ships.push(Ship::new(ShipType::Destroyer));
        ships.push(Ship::new(ShipType::Destroyer));
        ships.push(Ship::new(ShipType::Destroyer));
        ships.push(Ship::new(ShipType::Destroyer));
        
        return Player { board, ships }
    }

    // place ships
    // fire postion 
}
