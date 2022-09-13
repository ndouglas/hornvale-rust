use specs::prelude::*;
use std::collections::VecDeque;
use std::sync::Mutex;

use crate::events::Event;
use crate::traits::Eventable;

lazy_static! {
  pub static ref EVENT_QUEUE: Mutex<VecDeque<Event>> = Mutex::new(VecDeque::new());
}

#[named]
pub fn enqueue_event(event: Event) {
  EVENT_QUEUE.lock().unwrap().push_back(event);
}

#[named]
pub fn run_event_queue(ecs: &mut World) {
  loop {
    let event_option: Option<Event> = EVENT_QUEUE.lock().unwrap().pop_front();
    if let Some(event) = event_option {
      event.dispatch(ecs);
    } else {
      break;
    }
  }
}
