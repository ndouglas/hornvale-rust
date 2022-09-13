use specs::prelude::*;

pub trait Eventable {
  /// Dispatch this event.
  fn dispatch(&self, _ecs: &mut World) {
    todo!();
  }
}
