use colored::*;
use specs::prelude::*;

use crate::actions::*;
use crate::commands::Command;
use crate::ecs::components::*;
use crate::effects::Effect;
use crate::effects::MoveEntityEffect;
use crate::model::Direction;
use crate::queue::enqueue_effect;
use crate::queue::enqueue_message;
use crate::traits::Actionable;
use crate::traits::Commandable;
use crate::traits::WorldUsable;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct MoveDirectionAction {
  pub entity: Entity,
  pub direction: Direction,
}

impl Actionable for MoveDirectionAction {
  #[named]
  fn perform(&self, ecs: &mut World) {
    trace_enter!();
    if let Some(room_entity) = ecs.get_entity_room_entity(self.entity) {
      if let Some(exit) = ecs.get_room_entity_exit(room_entity, &self.direction) {
        let new_room_entity = exit.room_entity;
        enqueue_effect(Effect::MoveEntity(MoveEntityEffect {
          entity: self.entity,
          from: room_entity,
          to: new_room_entity,
        }));
      } else {
        enqueue_message(format!("{}", "You are unable to move in that direction!".red()));
      }
    } else {
      enqueue_message(format!("{}", "You are unable to move in that direction...".red()));
    }
    trace_exit!();
  }
}
