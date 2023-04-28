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

        ships.push(Ship::new(ShipType::Carrier), 0);
        ships.push(Ship::new(ShipType::Battleship), 1);
        ships.push(Ship::new(ShipType::Cruiser), 2);
        ships.push(Ship::new(ShipType::Cruiser), 3);
        ships.push(Ship::new(ShipType::Destroyer), 4);
        ships.push(Ship::new(ShipType::Destroyer), 5);
        ships.push(Ship::new(ShipType::Destroyer), 6);
        ships.push(Ship::new(ShipType::Destroyer), 7);
        
        return Player { board, ships }
    }
    fn isClear(self, tile: (u8, u8)) {

        bool test = self.board.isTileEmpty(tile);
        print(test)
    }
//    fn placeShips () {
        // niave solution randomly selected 
        //choose random location
        // is empty? 
        // - yes check valid orientations 
        // - no ^ back to choose random location 
 //   }

    // place ships
    // fire postion 
}
