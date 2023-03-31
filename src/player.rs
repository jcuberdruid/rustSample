use crate::ship::Ship;
use crate::ship::ShipType;

pub struct Player {
    pub ships: Vec<Ship>,
}

impl Player {
    pub fn setup(&mut self) {
        self.ships.push(Ship::new(ShipType::Carrier));
        self.ships.push(Ship::new(ShipType::Battleship));
        self.ships.push(Ship::new(ShipType::Cruiser));
        self.ships.push(Ship::new(ShipType::Cruiser));
        self.ships.push(Ship::new(ShipType::Destroyer));
        self.ships.push(Ship::new(ShipType::Destroyer));
        self.ships.push(Ship::new(ShipType::Destroyer));
        self.ships.push(Ship::new(ShipType::Destroyer));
    }
    // place ships
    // fire postion 
}
