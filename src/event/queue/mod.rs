use std::collections::VecDeque;
use std::sync::Mutex;

use super::Event;
use super::Eventable;

lazy_static! {
  pub static ref EVENT_QUEUE: Mutex<VecDeque<Event>> = Mutex::new(VecDeque::new());
}

#[named]
pub fn enqueue_event(event: Event) {
  EVENT_QUEUE.lock().unwrap().push_back(event);
}

#[named]
pub fn run_event_queue() {
  let events = EVENT_QUEUE.lock().unwrap().drain(..).collect::<Vec<Event>>();
  for event in events.iter() {
    event.dispatch();
  }
}
