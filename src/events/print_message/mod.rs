use specs::prelude::*;

use crate::traits::Eventable;

pub enum PrintMessageEvent {
  RoomDescription(Entity),
}

impl Eventable for PrintMessageEvent {
  /// Dispatch this event.
  fn dispatch(&self, ecs: &mut World) {
    use PrintMessageEvent::*;
    match self {
      RoomDescription(room) => {
        enq_message!(format_room!(ecs, *room));
      },
    }
  }
}
