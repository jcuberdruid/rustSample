

#[derive(Copy, Clone)]
pub enum ShipType {
    Carrier,
    Battleship,
    Cruiser,
    Destroyer,
}

impl ShipType {
    pub fn tile_count(&self) -> u16 {
        match self {
            ShipType::Carrier => 5,
            ShipType::Battleship => 4,
            ShipType::Cruiser => 3,
            ShipType::Destroyer => 1,
        }
    }
}

pub struct Ship {
    pub ship_type: ShipType,
    pub lives: u16,
}

impl Ship {
    pub fn new(ship_type: ShipType, ship_index: u16) -> Ship {
        Ship {
            ship_type,
            lives: ship_type.tile_count(),
        }
    }
}
