use specs::prelude::*;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

lazy_static! {
  pub static ref MESSAGE_QUEUE: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());
}

#[named]
pub fn enqueue_message(message: String) {
  trace_enter!();
  MESSAGE_QUEUE.lock().unwrap().push_back(message);
  trace_exit!();
}

#[named]
pub fn get_messages() -> Vec<String> {
  trace_enter!();
  let result = MESSAGE_QUEUE.lock().unwrap().drain(..).collect::<Vec<String>>();
  trace_exit!();
  result
}

#[named]
pub fn start_message_spammer() {
  trace_enter!();
  use rand::{thread_rng, Rng};
  use std::thread::{sleep, spawn};
  use std::time::Duration;
  spawn(move || {
    let mut rng = thread_rng();
    let mut i = 0usize;
    loop {
      enqueue_message(format!("External message #{}", i));
      let wait_ms = rng.gen_range(1000..20000);
      sleep(Duration::from_millis(wait_ms));
      i += 1;
    }
  });
  trace_exit!();
}
