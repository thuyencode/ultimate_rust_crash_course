use std::time::Duration;

use crate::{
  frame::{Drawable, Frame},
  shot::Shot,
  NUM_COLS, NUM_ROWS,
};

pub struct Player {
  x: usize,
  y: usize,
  shots: Vec<Shot>,
}

impl Player {
  pub fn new() -> Self {
    Self {
      x: NUM_COLS / 2,
      y: NUM_ROWS - 1,
      shots: Vec::new(),
    }
  }

  pub fn move_left(&mut self) {
    if self.x > 0 {
      self.x -= 1;
    }
  }

  pub fn move_right(&mut self) {
    if self.x < NUM_COLS - 1 {
      self.x += 1;
    }
  }

  pub fn shoot(&mut self) -> bool {
    if self.shots.len() < 2 {
      self.shots.push(Shot::new(self.x, self.y - 1));
      true
    } else {
      false
    }
  }

  pub fn update(&mut self, duration: Duration) {
    self.shots.iter_mut().for_each(|shot: &mut Shot| {
      shot.update(duration);
    });

    self.shots.retain(|shot: &Shot| !shot.is_dead());
  }
}

impl Drawable for Player {
  fn draw(&self, frame: &mut Frame) {
    frame[self.x][self.y] = "A";

    self.shots.iter().for_each(|shot: &Shot| shot.draw(frame));
  }
}
