use rand::seq::SliceRandom;
use rand::Rng;

use crate::ship::Ship;
use crate::ship::ShipType;
use crate::tile::Board;

pub struct Player {
    pub ships: Vec<Ship>,
    pub board: Board,
}

impl Default for Player {
    fn default() -> Player { 
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
        
        let mut player =  Player { board, ships };
 
        for n in 0.. player.ships.len() {
            player.placeShip(player.ships[n].lives as u8, n as u16);
        }


        return player
    }
}

impl Player { 

   pub fn isClear(&mut self, x: u32, y: u32) -> bool {
    
         let test: bool = self.board.isTileEmpty(x, y);
         return test; 
     }

   pub fn setTile(&mut self, x: u32, y: u32, sI: u16) {
         self.board.setTile(x, y, sI);
     }

     
    pub fn chooseRandRangeShipLength(&self,shipLength: u8) -> u8 {
            return rand::thread_rng().gen_range(1..(11 - (shipLength - 1))) as u8;
     }

    pub fn placeShip(&mut self, shipLength: u8, shipIndex: u16) {

        let c1 = rand::thread_rng().gen_range(0..self.chooseRandRangeShipLength(shipLength));
        let c2 = rand::thread_rng().gen_range(0..rand::thread_rng().gen_range(1..11));
        let rand = rand::thread_rng().gen_range(0..2);

        let mut x: u32 = 0; 
        let mut y: u32 = 0; 

        let mut coordinates:Vec<(u32, u32)> = Vec::new();

        let mut direction = "";

        if rand == 1 {
            direction = "right";  
            x = c1 as u32; 
            y = c2 as u32; 
            
            for i in 0..shipLength as u32 {
                if self.isClear(x+i, y) {
                    coordinates.push((x+i, y));
                } else {
                    println!("Placement blocked... retrying...");
                    self.placeShip(shipLength, shipIndex); 
                    return
                }
            }

       } else if rand == 0 {
            direction = "down";
            x = c2 as u32; 
            y = c1 as u32; 

            for i in 0..shipLength as u32{
                if self.isClear(x, y+i) {
                    coordinates.push((x, y+i));
                } else {
                    println!("Placement blocked... retrying...");
                    self.placeShip(shipLength, shipIndex); 
                    return
                }
            }
        }

        print!("ship of {} length is placed at ",shipLength);
        for n in coordinates {
            self.setTile(n.0,n.1,shipIndex);
            print!("({},{}),", n.0, n.1);
        }
        println!(", in the {} direction", direction);

     }

}
