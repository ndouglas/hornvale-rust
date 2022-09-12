use colored::*;
use specs::prelude::*;

use crate::actions::*;
use crate::commands::Command;
use crate::ecs::components::*;
use crate::model::Direction;
use crate::queue::enqueue_message;
use crate::traits::Commandable;
use crate::traits::Effectable;
use crate::traits::WorldUsable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct LookEffect {
  pub entity: Entity,
}

impl Effectable for LookEffect {
  #[named]
  fn execute(&self, ecs: &mut World) {
    trace_enter!();
    if let Some(room_entity) = ecs.get_entity_room_entity(self.entity) {
      if let Some(name) = ecs.get_entity_name(room_entity) {
        enq_message!(format!("{}", name.magenta()));
      }
      if let Some(description) = ecs.get_entity_description(room_entity) {
        enq_message!(format!("{}", description.bright_green()));
      }
      if let Some(exits) = ecs.get_room_entity_exits(room_entity) {
        enq_message!(format!("{}", exits));
      }
    }
    trace_exit!();
  }
}
