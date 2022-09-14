use specs::prelude::*;

use super::Eventable;

use crate::state::STATE;

pub enum PrintMessageEvent {
  RoomDescription(Entity),
}

impl Eventable for PrintMessageEvent {
  /// Dispatch this event.
  fn dispatch(&self) {
    use PrintMessageEvent::*;
    match self {
      RoomDescription(room) => {
        enq_message!(format_room!(&STATE.lock().unwrap().ecs, *room));
      },
    }
  }
}
