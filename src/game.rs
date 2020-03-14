mod board;

use board::Board;
use std::io::{stdin,stdout,Write};

pub struct Game {
    player0: i8, // count n plays of player0, player0 always start
    player1: i8,
    board: Board
}

impl Game {
  pub fn new() -> Self {
    Game {
        player0: 0,
        player1: 0,
        board: Board::new()
    }
  }

  pub fn run(&self) {
    let mut done = false;
    loop {
      self.turn_player();
      if done {
        break;
      }
    }
  }
}

impl Game {
  fn turn_player(&self) {
    let player_turn = if self.player0 >= self.player1 {&self.player0} else {&self.player1};
    println!("\n\t It is your turn Player{}\n", player_turn);
    self.board.print_board();
    let result = self.get_input();
    println!("result: {}", result);
  }

  fn get_input(&self) -> String {
    let mut s=String::new();
    println!("Insert position X Y (e.g. 1 2)");
    print!("> ");
    let _= stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    s
  }
}
