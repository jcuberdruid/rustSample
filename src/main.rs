mod ship;
mod player;

use flo_draw::canvas::*;
use flo_draw::*;
use crate::player::Player;

fn main() {
    let mut player_one = Player { ships: Vec::new() };
    player_one.setup(); 
}
