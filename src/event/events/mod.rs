use std::collections::VecDeque;
use std::sync::Mutex;

pub mod action;
pub use action::*;
pub mod print_message;
pub use print_message::*;

pub trait Eventable {
  /// Dispatch this event.
  fn dispatch(&self) {
    todo!();
  }
}

pub enum Event {
  /// An event (could|will|did) (happen|not happen).
  Action(ActionEvent),
  /// A message will be printed to present to the player.
  PrintMessage(PrintMessageEvent),
}

impl Eventable for Event {
  /// Dispatch this event.
  fn dispatch(&self) {
    use Event::*;
    match self {
      Action(event) => event.dispatch(),
      PrintMessage(event) => event.dispatch(),
    }
  }
}

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
