use specs::prelude::*;

use crate::state::STATE;

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
    let ecs = &STATE.lock().unwrap().ecs;
    move_entity_room!(ecs, self.entity, self.to);
    enq_effect!(eff_print_room!(self.to));
  }
}
