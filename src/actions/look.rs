use specs::prelude::*;

use crate::effects::*;
use crate::traits::Actionable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct LookAction {
  pub entity: Entity,
}

impl Actionable for LookAction {
  #[named]
  fn attempt(&self, ecs: &mut World) {
    if let Some(room) = get_current_room!(ecs, self.entity) {
      enq_effect!(eff_print_room!(room));
    }
  }
}
