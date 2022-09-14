use specs::prelude::*;

use crate::effect::Effect;
use crate::effect::MoveEntityEffect;
use crate::effect::PrintErrorEffect;
use crate::model::Direction;
use crate::state::STATE;

use super::super::Actionable;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct MoveDirectionAction {
  pub entity: Entity,
  pub direction: Direction,
}

impl Actionable for MoveDirectionAction {
  #[named]
  fn attempt(&self) {
    let ecs = &STATE.lock().unwrap().ecs;
    let room_entity = get_current_room!(ecs, self.entity).unwrap();
    match get_exit_to!(ecs, room_entity, &self.direction) {
      Some(exit) => enq_effect!(eff_move_entity!(self.entity, room_entity, exit.room_entity)),
      None => enq_effect!(eff_print_error!("You are unable to move in that direction!".to_string())),
    }
  }
}
