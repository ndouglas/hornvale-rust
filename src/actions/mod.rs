use specs::prelude::*;

use crate::traits::Actionable;

pub mod look;
pub use look::*;
pub mod move_direction;
pub use move_direction::*;

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Action {
  Look(LookAction),
  MoveDirection(MoveDirectionAction),
}

impl Actionable for Action {
  #[named]
  fn attempt(&self, ecs: &mut World) {
    use Action::*;
    match self {
      Look(action) => action.attempt(ecs),
      MoveDirection(action) => action.attempt(ecs),
    }
  }
}
