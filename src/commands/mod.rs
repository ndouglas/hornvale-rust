pub mod look;
pub use look::*;
pub mod move_compass_direction;
pub use move_compass_direction::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum Command {
  Look(Look),
  MoveCompassDirection(MoveCompassDirection),
}
