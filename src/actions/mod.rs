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

impl Actionable for Action {}
