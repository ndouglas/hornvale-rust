use crate::entity::Entity;
use crate::room::Room;

use super::super::Effectable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct MoveEntityEffect {
  pub entity: Entity,
  pub from: Room,
  pub to: Room,
}

impl Effectable for MoveEntityEffect {
  #[named]
  fn execute(&self) {
    move_entity_room!(self.entity, self.to);
    enq_effect!(eff_print_room!(self.to));
  }
}
