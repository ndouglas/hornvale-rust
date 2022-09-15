use crate::entity::Entity;

use super::Eventable;

pub enum PrintMessageEvent {
  RoomDescription(Entity),
}

impl Eventable for PrintMessageEvent {
  /// Dispatch this event.
  fn dispatch(&self) {
    use PrintMessageEvent::*;
    match self {
      RoomDescription(room) => {
        enq_message!(format_room!(*room));
      },
    }
  }
}
