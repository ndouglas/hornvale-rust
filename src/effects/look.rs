use specs::prelude::*;

use crate::events::Event;
use crate::events::PrintMessageEvent;

use crate::traits::Effectable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct LookEffect {
  pub entity: Entity,
}

impl Effectable for LookEffect {
  #[named]
  fn execute(&self, ecs: &mut World) {
    if let Some(room) = get_current_room!(ecs, self.entity) {
      enq_event!(Event::PrintMessage(PrintMessageEvent::RoomDescription(room)));
    }
  }
}
