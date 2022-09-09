use specs::prelude::*;

use crate::actions::*;
use crate::commands::Command;
use crate::ecs::components::*;
use crate::model::CompassDirection;
use crate::traits::Actionable;
use crate::traits::Commandable;
use crate::traits::WorldUsable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct LookAction {
  pub entity: Entity,
}

impl Actionable for LookAction {
  #[named]
  fn perform(&self, ecs: &mut World) {
    trace_enter!();
    if let Some(room_entity) = ecs.get_entity_room_entity(self.entity) {
      if let Some(name) = ecs.get_entity_name(room_entity) {
        print!("{}\n", name);
      }
      if let Some(description) = ecs.get_entity_description(room_entity) {
        print!("{}\n", description);
      }
    }
    trace_exit!();
  }
}
