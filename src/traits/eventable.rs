use specs::prelude::*;

pub trait Eventable {
  /// Dispatch this event.
  fn dispatch(&self, ecs: &mut World);
}
