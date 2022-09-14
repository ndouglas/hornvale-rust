use std::fmt;
use std::sync::Mutex;
use std::thread::{sleep, spawn};
use std::time::Duration;

use crate::action::run_action_queue;
use crate::command::run_command_queue;
use crate::effect::run_effect_queue;
use crate::event::run_event_queue;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Tick(pub u64);

impl fmt::Display for Tick {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

lazy_static! {
  pub static ref TICK: Mutex<Tick> = Mutex::new(Tick(0));
}

#[named]
pub fn manual_tick() {
  run_command_queue();
  run_action_queue();
  run_effect_queue();
  run_event_queue();
}

#[named]
pub fn start_tick() {
  spawn(move || loop {
    {
      let mut tick = TICK.lock().unwrap();
      tick.0 += 1;
      manual_tick();
    }
    sleep(Duration::from_millis(50));
  });
}
