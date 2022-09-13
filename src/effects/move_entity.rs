use specs::prelude::*;

use crate::traits::Effectable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct MoveEntityEffect {
  pub entity: Entity,
  pub from: Entity,
  pub to: Entity,
}

impl Effectable for MoveEntityEffect {
  #[named]
  fn execute(&self, ecs: &mut World) {
    move_entity_room!(ecs, self.entity, self.to);
    enq_effect!(eff_print_room!(self.to));
  }
}
