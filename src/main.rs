#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

mod ship;
mod player;
mod tile; 

use flo_draw::canvas::*;
use flo_draw::*;
use crate::player::Player;

fn main() {
    let mut player_one = Player::default(); 
    player_one.board.printBoard(); 
}
