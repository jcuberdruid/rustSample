mod ship;
mod player;
mod tile; 

use flo_draw::canvas::*;
use flo_draw::*;
use crate::player::Player;

fn main() {
//    let mut player_one = Player { ships: Vec::new(), board: board};
    print!("test");
    let mut player_one = Player::default(); 
    player_one.isTileEmpty(tile: (5, 5));
}
