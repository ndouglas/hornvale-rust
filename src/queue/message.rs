use specs::prelude::*;
use std::collections::VecDeque;
use std::sync::Mutex;

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
  let result = MESSAGE_QUEUE.lock().unwrap().drain(0..).collect::<Vec<String>>();
  trace_exit!();
  result
}
