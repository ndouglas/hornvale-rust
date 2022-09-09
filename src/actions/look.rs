use specs::prelude::*;

use crate::actions::*;
use crate::commands::Command;
use crate::effects::*;
use crate::ecs::components::*;
use crate::model::CompassDirection;
use crate::queue::enqueue_effect;
use crate::traits::Actionable;
use crate::traits::Commandable;
use crate::traits::WorldUsable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct LookAction {
  pub entity: Entity,
}

impl Actionable for LookAction {
  #[named]
  fn perform(&self, ecs: &mut World) {
    trace_enter!();
    enqueue_effect(Effect::Look(LookEffect{ entity: self.entity }));
    trace_exit!();
  }
}
