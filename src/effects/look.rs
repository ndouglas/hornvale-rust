use colored::*;
use specs::prelude::*;

use crate::actions::*;
use crate::commands::Command;
use crate::ecs::components::*;
use crate::model::CompassDirection;
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
        enqueue_message(format!("{}\n", name.magenta()));
      }
      if let Some(description) = ecs.get_entity_description(room_entity) {
        enqueue_message(format!("{}\n", description.bright_green()));
      }
    }
    trace_exit!();
  }
}
