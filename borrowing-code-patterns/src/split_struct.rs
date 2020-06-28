pub struct Stats {
  hp: u8,
  sp: u8,
}

pub struct Monster {
  stats: Stats,
  frients: Vec<Friend>,
}

pub struct Friend {
  loyalty: u8,
}

impl Friend {
  pub fn new() -> Self {
    Friend {
      loyalty: 0
    }
  }
}

impl Monster {
  pub fn final_breath(&mut self) {
    if let Some(firend) = self.frients.first() {
      self.stats.heal(firend.loyalty);
      println!("healing for {}", firend.loyalty);
    }
  }

  pub fn new() -> Self {
    Monster {
      stats: Stats{
        hp: 100,
        sp: 100,
      },
      frients: vec![Friend::new()]
    }
  }
}

impl Stats {
  pub fn heal(&mut self, amount: u8) {
    self.hp += amount;
    self.sp -= amount;
  }
}