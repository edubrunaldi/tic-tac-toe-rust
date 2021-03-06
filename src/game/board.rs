
pub struct Board {
  circle: char,
  cross_mark: char,
  spaces: Vec<char>,
}

pub enum Draw {
  Circle,
  CrossMark
}

impl Board {
  pub fn new() -> Self {
    Board {
      circle: '\u{2b55}',
      cross_mark: '\u{2694}',
      spaces: vec!(' '; 9),
    }
  }
  pub fn print_board(&self) {
    println!("\t\t    1   2   3 \n");
    println!("\t\t1   {} | {} | {} ", self.spaces[0], self.spaces[1], self.spaces[2]);
    println!("\t\t   -----------");
    println!("\t\t2   {} | {} | {} ", self.spaces[3], self.spaces[4], self.spaces[5]);
    println!("\t\t   -----------");
    println!("\t\t3   {} | {} | {} \n", self.spaces[6], self.spaces[7], self.spaces[8]);
  }

  pub fn draw(&mut self, row: &usize, col: &usize, item: Draw) {
    match item {
      Draw::Circle => self.spaces[(row-1)*3 + (col-1) ] = self.circle,
      Draw::CrossMark => self.spaces[(row-1)*3 + (col-1) ] = self.cross_mark
    };
  }

  pub fn is_valid_space(&self, row: &usize, col: &usize) -> bool {
    self.spaces[(row-1)*3 + (col-1)] == ' '
  }

  // There is only 8 ways to win
  // 3 rows
  // 3 cols
  // 2 diagonal
  pub fn someone_win(&self) -> bool {
    // rows
    (self.spaces[0] != ' ' && self.spaces[0] == self.spaces[1] && self.spaces[1] == self.spaces[2] && self.spaces[0] == self.spaces[2]) ||
    (self.spaces[3] != ' ' && self.spaces[3] == self.spaces[4] && self.spaces[4] == self.spaces[5] && self.spaces[3] == self.spaces[5]) ||
    (self.spaces[6] != ' ' && self.spaces[6] == self.spaces[7] && self.spaces[7] == self.spaces[8] && self.spaces[6] == self.spaces[8]) ||
    // cols
    (self.spaces[0] != ' ' && self.spaces[0] == self.spaces[3] && self.spaces[3] == self.spaces[6] && self.spaces[0] == self.spaces[6]) ||
    (self.spaces[1] != ' ' && self.spaces[1] == self.spaces[4] && self.spaces[4] == self.spaces[7] && self.spaces[1] == self.spaces[7]) ||
    (self.spaces[2] != ' ' && self.spaces[2] == self.spaces[5] && self.spaces[5] == self.spaces[8] && self.spaces[2] == self.spaces[8]) ||
    // left to right diagonal
    (self.spaces[0] != ' ' && self.spaces[0] == self.spaces[4] && self.spaces[4] == self.spaces[8] && self.spaces[0] == self.spaces[8]) ||
    // right to left diagonal
    (self.spaces[2] != ' ' && self.spaces[2] == self.spaces[4] && self.spaces[4] == self.spaces[6] && self.spaces[2] == self.spaces[6]) 
  }

  pub fn reset(&mut self) {
    for i in &mut self.spaces {
      *i = ' ';
    }
  }
}
