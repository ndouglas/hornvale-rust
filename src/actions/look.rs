use specs::prelude::*;

use crate::effects::*;
use crate::traits::Actionable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct LookAction {
  pub entity: Entity,
}

impl Actionable for LookAction {
  #[named]
  fn perform(&self, _ecs: &mut World) {
    enq_effect!(eff_look!(self.entity));
  }
}
