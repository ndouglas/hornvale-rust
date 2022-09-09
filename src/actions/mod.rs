use specs::prelude::*;

use crate::traits::Actionable;

pub mod look;
pub use look::*;
pub mod move_compass_direction;
pub use move_compass_direction::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum Action {
  Look(LookAction),
  MoveCompassDirection(MoveCompassDirectionAction),
}

impl Actionable for Action {
  #[named]
  fn should_perform(&self, ecs: &mut World) -> bool {
    trace_enter!();
    use Action::*;
    let result = match self {
      Look(action) => action.should_perform(ecs),
      MoveCompassDirection(action) => action.should_perform(ecs),
    };
    trace_exit!();
    result
  }

  #[named]
  fn can_perform(&self, ecs: &mut World) -> bool {
    trace_enter!();
    use Action::*;
    let result = match self {
      Look(action) => action.can_perform(ecs),
      MoveCompassDirection(action) => action.can_perform(ecs),
    };
    trace_exit!();
    result
  }

  #[named]
  fn perform(&self, ecs: &mut World) {
    trace_enter!();
    use Action::*;
    match self {
      Look(action) => action.perform(ecs),
      MoveCompassDirection(action) => action.perform(ecs),
    }
    trace_exit!();
  }
}
