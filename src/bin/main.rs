extern crate tic_tac_toe_rust;

use tic_tac_toe_rust::game::Game;

fn main() {
  let mut game = Game::new();
  loop {
    game.run();
  }
}
