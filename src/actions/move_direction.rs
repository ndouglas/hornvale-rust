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
  fn can_perform(&self, ecs: &mut World) -> bool {
    let room_entity = get_current_room!(ecs, self.entity).unwrap();
    None != get_exit_to!(ecs, room_entity, &self.direction)
  }

  #[named]
  fn perform(&self, ecs: &mut World) {
    let room_entity = get_current_room!(ecs, self.entity).unwrap();
    if let Some(exit) = get_exit_to!(ecs, room_entity, &self.direction) {
      enq_effect!(eff_move_entity!(self.entity, room_entity, exit.room_entity));
    } else {
      enq_message!(format!("{}", "You are unable to move in that direction!".red()));
    }
  }
}
