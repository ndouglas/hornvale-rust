pub mod events;
pub use events::*;
pub mod queue;
pub use queue::*;

pub trait Eventable {
  /// Dispatch this event.
  fn dispatch(&self) {
    todo!();
  }
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Event {
  /// An action (could|will|did) (happen|not happen).
  Action(ActionEvent),
  /// An effect (will|did) happen.
  Effect(EffectEvent),
}

impl Eventable for Event {
  /// Dispatch this event.
  fn dispatch(&self) {
    use Event::*;
    match self {
      Action(event) => event.dispatch(),
      Effect(event) => event.dispatch(),
    }
  }
}
