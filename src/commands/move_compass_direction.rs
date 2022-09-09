use specs::prelude::*;

use crate::actions::*;
use crate::commands::Command;
use crate::ecs::components::*;
use crate::model::CompassDirection;
use crate::traits::Actionable;
use crate::traits::Commandable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct MoveCompassDirectionCommand {
  pub entity: Entity,
  pub compass_direction: CompassDirection,
}

impl Commandable for MoveCompassDirectionCommand {
  #[named]
  fn get_action(&self) -> Action {
    trace_enter!();
    let result = Action::MoveCompassDirection(MoveCompassDirectionAction {
      entity: self.entity,
      compass_direction: self.compass_direction,
    });
    trace_exit!();
    result
  }
}
