mod board;
mod utils;
use board::{Board, Draw};

pub struct Game {
    player0: i8, // count n plays of player0, player0 always start
    player1: i8,
    finished: bool,
    board: Board
}

impl Game {
  pub fn new() -> Self {
    Game {
        player0: 0,
        player1: 0,
        finished: false,
        board: Board::new()
    }
  }

  pub fn run(&mut self) {
    loop {
      self.player_turn();
      if self.finished {
        break;
      }
    }
  }
}

impl Game {
  fn player_turn(&mut self) {
    let player_turn = if self.player0 >= self.player1 {&self.player0} else {&self.player1};
    println!("\n\t It is your turn Player{}\n", player_turn);
    self.board.print_board();
    let mut result = utils::get_input("Insert position X Y (e.g > 1 2)".to_string());
    result = result.trim().to_string();
    while !utils::validate_regex("1 2", &result) {
      result = utils::get_input(format!("({}) is invalid position, try (> row col, e.g. > 1 2)", result));
      result = result.trim().to_string();
    }
    self.do_action(result);
  }

  fn do_action(&mut self, action: String) {
    let position: Vec<&str> = action.split_ascii_whitespace().collect();
    let row: &usize = &position[0].parse::<usize>().unwrap();
    let col: &usize = &position[1].parse::<usize>().unwrap();
    self.board.draw(*row, *col, Draw::Circle);
  }
}
