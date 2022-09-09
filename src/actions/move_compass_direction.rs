use specs::prelude::*;

use crate::actions::*;
use crate::commands::Command;
use crate::ecs::components::*;
use crate::effects::Effect;
use crate::effects::MoveEntityEffect;
use crate::queue::enqueue_effect;
use crate::model::CompassDirection;
use crate::traits::Actionable;
use crate::traits::Commandable;
use crate::traits::WorldUsable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct MoveCompassDirectionAction {
  pub entity: Entity,
  pub compass_direction: CompassDirection,
}

impl Actionable for MoveCompassDirectionAction {
  #[named]
  fn perform(&self, ecs: &mut World) {
    trace_enter!();
    if let Some(room_entity) = ecs.get_entity_room_entity(self.entity) {
      if let Some(exit) = ecs.get_room_entity_exit(room_entity, self.compass_direction) {
        let new_room_entity = exit.room_entity;
        enqueue_effect(Effect::MoveEntity(MoveEntityEffect {
          entity: self.entity,
          from: room_entity,
          to: new_room_entity,
        }));
      }
      else {
        print!("Somebody is unable to move in that direction!\n");
      }
    } else {
      print!("Somebody is unable to move in that direction...\n");
    }
    trace_exit!();
  }
}
