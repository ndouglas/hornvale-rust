use specs::prelude::*;

use crate::actions::*;
use crate::commands::Command;
use crate::ecs::components::*;
use crate::model::CompassDirection;
use crate::traits::Actionable;
use crate::traits::Commandable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct LookCommand {}

impl Commandable for LookCommand {
  #[named]
  fn get_action(&self) -> Action {
    trace_enter!();
    let result = Action::Look(LookAction {});
    trace_exit!();
    result
  }
}
