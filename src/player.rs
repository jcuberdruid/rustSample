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

        ships.push(Ship::new(ShipType::Carrier, 0));
        ships.push(Ship::new(ShipType::Battleship, 1));
        ships.push(Ship::new(ShipType::Cruiser, 2));
        ships.push(Ship::new(ShipType::Cruiser, 3));
        ships.push(Ship::new(ShipType::Destroyer, 4));
        ships.push(Ship::new(ShipType::Destroyer, 5));
        ships.push(Ship::new(ShipType::Destroyer, 6));
        ships.push(Ship::new(ShipType::Destroyer, 7));
        
        return Player { board, ships }
    }
}

impl Player { 
  pub fn isClear(&mut self, x: u32, y: u32) {
        let test: bool = self.board.isTileEmpty(x, y);
        println!("is the tile clear? {}", test); 
    }
 pub fn setTile(&mut self, x: u32, y: u32, sI: u16) {
        self.board.setTile(x, y, sI);
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
