use std::fmt;
use std::sync::Mutex;
use std::thread::{sleep, spawn};
use std::time::Duration;

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
pub fn start_tick() {
  spawn(move || {
    loop {
      {
        let mut tick = TICK.lock().unwrap();
        tick.0 += 1;  
        enq_message!(format!("Tick #{}", tick.0));
      }
      sleep(Duration::from_secs(1));
    }
  });
}
