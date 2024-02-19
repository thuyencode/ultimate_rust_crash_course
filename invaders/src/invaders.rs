use crate::{
  frame::{Drawable, Frame},
  NUM_COLS, NUM_ROWS,
};
use rusty_time::Timer;
use std::{cmp::max, time::Duration};

pub struct Invader {
  x: usize,
  y: usize,
}

pub struct Invaders {
  pub army: Vec<Invader>,
  move_timer: Timer,
  direction: i32,
}

impl Invaders {
  pub fn new() -> Self {
    let mut army: Vec<Invader> = Vec::new();

    for x in 0..NUM_COLS {
      for y in 0..NUM_ROWS {
        if (x > 1) && (x < NUM_COLS - 2) && (y > 0) && (y < 9) && (x % 2) == 0 && (y % 2 == 0) {
          army.push(Invader { x, y });
        }
      }
    }

    Self {
      army,
      move_timer: Timer::new(Duration::from_millis(2000)),
      direction: 1,
    }
  }

  pub fn update(&mut self, delta: Duration) -> bool {
    self.move_timer.tick(delta);

    if self.move_timer.finished() {
      self.move_timer.reset();

      let mut downwards: bool = false;

      if self.direction == -1 {
        let min_x: usize = self
          .army
          .iter()
          .map(|invader: &Invader| invader.x)
          .min()
          .unwrap_or(0);

        if min_x == 0 {
          self.direction = 1;
          downwards = true;
        }
      } else {
        let max_x: usize = self
          .army
          .iter()
          .map(|invader: &Invader| invader.x)
          .max()
          .unwrap_or(0);

        if max_x == NUM_COLS - 1 {
          self.direction = -1;
          downwards = true;
        }
      }

      if downwards {
        let new_duration: u128 = max(self.move_timer.duration().as_millis() - 250, 250);

        self.move_timer = Timer::new(Duration::from_millis(new_duration as u64));

        self
          .army
          .iter_mut()
          .for_each(|invader: &mut Invader| invader.y += 1);
      } else {
        self.army.iter_mut().for_each(|invader: &mut Invader| {
          invader.x = ((invader.x as i32) + self.direction) as usize
        });
      }

      return true;
    }

    false
  }
}

impl Drawable for Invaders {
  fn draw(&self, frame: &mut Frame) {
    self.army.iter().for_each(|invader: &Invader| {
      frame[invader.x][invader.y] = if (self.move_timer.remaining().as_secs_f32()
        / self.move_timer.duration().as_secs_f32())
        > 0.5
      {
        "x"
      } else {
        "+"
      }
    })
  }
}
