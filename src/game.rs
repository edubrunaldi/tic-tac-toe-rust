mod board;
mod utils;
use board::{Board, Draw};

pub struct Game {
    player0: i8, // count n plays of player0, player0 always start
    player1: i8,
    finished: bool,
    board: Board,
    input_regex: String
}

impl Game {
  pub fn new() -> Self {
    Game {
        player0: 0,
        player1: 0,
        finished: false,
        board: Board::new(),
        input_regex: r"^[1-3] [1-3]$".to_string()
    }
  }

  pub fn run(&mut self) {
    loop {
      let result = self.menu();
      match result {
        0 => break,
        1 => {
          self.start_game();
          self.board.reset();
        },
        _ => println!("choose between 0 and 1")
      };
      
    }
  }
}

impl Game {

  fn menu(&self) -> i8{
    println!("\n\nWelcome to tic-tac-toe created with Rust");
    println!("created by Xima\n");
    println!("actions:");
    println!("0) Quit game");
    println!("1) Let's play");
    utils::get_input("".to_string()).trim().parse::<i8>().unwrap()
  }
  fn start_game(&mut self) {
    loop {
      self.player_turn();
      if self.finished {
        break;
      }
    }
    self.board.print_board();
    if self.board.someone_win() {
      let player_won: i8 = if self.player0 > self.player1 {0} else {1};

      println!("\t\n Congratulation Player{}, you won {}!!", player_won, '\u{1f3c6}');
    } else {
      println!("\t\n Oh! You guys draw =/");
    }
  }

  fn player_turn(&mut self) {
    let player_turn: i8 = if self.player0 <= self.player1 {0} else {1};
    println!("\n\t It is your turn Player{}\n", player_turn);
    self.board.print_board();
    let mut result = utils::get_input("Insert position X Y (e.g > 1 2)".to_string());
    result = result.trim().to_string();
    let values: &Vec<&str> = &result.split_ascii_whitespace().collect();
    let mut row: usize = values[0].parse::<usize>().unwrap();
    let mut col: usize = values[1].parse::<usize>().unwrap();
    loop {
      let input_msg: String;
      if !utils::validate_regex(&self.input_regex, &result) {
        input_msg = format!("({}) is invalid position, try (> row col, e.g. > 1 2)", result);
      } else if !self.board.is_valid_space(&row, &col) {
        input_msg = format!("({}) already used, try another position", result);
      } else {
        break;
      }
      result = utils::get_input(input_msg);
      result = result.trim().to_string();
      let values: &Vec<&str> = &result.split_ascii_whitespace().collect();
      let (update_row, update_col) = self.get_position(values);
      row = update_row;
      col = update_col;
    }

    self.do_action(&row, &col, player_turn);

    // update played turn
    if self.player0 <= self.player1 {
      self.player0 += 1;
    } else {
      self.player1 += 1;
    }

    // update finished variable
    self.finished = self.is_finished();
  }

  fn get_position(&self, values: &Vec<&str>) -> (usize, usize) {
    let row: usize = values.clone()[0].parse::<usize>().unwrap();
    let col: usize = values[1].parse::<usize>().unwrap();
    (row, col)
  }

  fn do_action(&mut self, row: &usize, col: &usize, player_turn: i8) {
    let item: Draw = if player_turn == 0 {Draw::Circle} else {Draw::CrossMark};
    self.board.draw(row, col, item);
  }

  fn is_finished(&self) -> bool {
    self.player0 + self.player1 >= 9 || self.board.someone_win()
  }
}
