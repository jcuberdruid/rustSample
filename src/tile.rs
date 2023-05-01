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
    pub fn isTileEmpty(&mut self, x: u32, y: u32) -> bool { 
        // yes -> set tile used, index = ship_index, hit = false, ret true 
        let thisTile: Tile = self.grid[(x*y) as usize];
        
        if let Tile::Empty = thisTile {
            return true; 
        }
        return false; 
    }
    pub fn setTile(&mut self, x: u32, y: u32, sI: u16) { 
        let thisTile: Tile = Tile::Used{shipIndex: sI, hit: false};
        self.grid[((y*10)+x) as usize] = thisTile; 
    }
    pub fn printBasic(&self) {
        let mut i = 0; 
       for n in &self.grid {
           match n {
                Tile::Empty => println!("{} is Empty", i),
                Tile::Used {shipIndex, hit} => println!("{} is used", i),
           }
       }
    }
    pub fn printBoard(&self) {
        let mut row: String;
        println!("board:");
        println!("  0 1 2 3 4 5 6 7 8 9");
        for y in 0..10 {
            row = String::from(" ");
            for x in 0..10 {
                let thisTile: &Tile = &self.grid[(x + y * 10) as usize];
                match thisTile {
                    Tile::Empty => row.push_str("+ "),
                    Tile::Used {shipIndex, hit} => {
                        if *hit {
                            row.push_str("H ");
                        } else {
                            row.push_str(&format!("{} ", shipIndex));
                        }
                    },
                }
            }
            println!("{}{}", y, row);
        }
    }
}

