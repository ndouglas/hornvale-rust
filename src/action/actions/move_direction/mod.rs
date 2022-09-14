use crate::effect::Effect;
use crate::effect::MoveEntityEffect;
use crate::effect::PrintErrorEffect;
use crate::entity::Entity;
use crate::model::Direction;

use super::super::Actionable;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct MoveDirectionAction {
  pub entity: Entity,
  pub direction: Direction,
}

impl Actionable for MoveDirectionAction {
  #[named]
  fn attempt(&self) {
    let room = get_current_room!(self.entity).unwrap();
    match get_exit_to!(room, &self.direction) {
      Some(exit) => enq_effect!(eff_move_entity!(self.entity, room, exit.room_entity)),
      None => enq_effect!(eff_print_error!("You are unable to move in that direction!".to_string())),
    }
  }
}
