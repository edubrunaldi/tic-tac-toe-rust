
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
      circle: '\u{26db}',
      cross_mark: '\u{274c}',
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

  pub fn draw(&mut self, row: usize, col: usize, item: Draw) {
    match item {
      Draw::Circle => self.spaces[(row-1)*3 + (col-1) ] = self.circle,
      Draw::CrossMark => self.spaces[(row-1)*3 + (col-1) ] = self.cross_mark
    };
  }
}
