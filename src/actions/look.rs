use specs::prelude::*;

use crate::state::STATE;
use super::Actionable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct LookAction {
  pub entity: Entity,
}

impl Actionable for LookAction {
  #[named]
  fn attempt(&self) {
    let ecs = &STATE.lock().unwrap().ecs;
    if let Some(room) = get_current_room!(ecs, self.entity) {
      enq_effect!(eff_print_room!(room));
    }
  }
}
