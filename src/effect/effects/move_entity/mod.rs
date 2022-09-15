use crate::entity::Entity;

use super::super::Effectable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct MoveEntityEffect {
  pub entity: Entity,
  pub from: Entity,
  pub to: Entity,
}

impl Effectable for MoveEntityEffect {
  #[named]
  fn execute(&self) {
    ent_move!(self.entity, self.to);
    enq_effect!(eff_print_room!(self.to));
  }
}
