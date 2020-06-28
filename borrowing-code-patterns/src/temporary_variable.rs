pub struct Player {
  score: i32
}

impl Player {
  pub fn set_score(&mut self, new_score: i32) {
    self.score = new_score;
  }

  pub fn score(&self) -> i32 {
    self.score
  }

  pub fn new() -> Self {
    Player{
      score: 0
    }
  }
}