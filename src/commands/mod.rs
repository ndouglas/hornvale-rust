pub mod move_compass_direction;
pub use move_compass_direction::*;

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Command {
  MoveCompassDirection(MoveCompassDirection),
}