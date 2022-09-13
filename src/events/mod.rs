use specs::prelude::*;

use crate::traits::Eventable;

pub mod action;
pub use action::*;
pub mod print_message;
pub use print_message::*;

pub enum Event {
  /// An event (could|will|did) (happen|not happen).
  Action(ActionEvent),
  /// A message will be printed to present to the player.
  PrintMessage(PrintMessageEvent),
}

impl Eventable for Event {
  /// Dispatch this event.
  fn dispatch(&self, ecs: &mut World) {
    use Event::*;
    match self {
      Action(event) => event.dispatch(ecs),
      PrintMessage(event) => event.dispatch(ecs),
    }
  }
}
