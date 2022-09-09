use crate::actions::Action;
use crate::traits::Commandable;

pub mod look;
pub use look::*;
pub mod move_compass_direction;
pub use move_compass_direction::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum Command {
  Look(LookCommand),
  MoveCompassDirection(MoveCompassDirectionCommand),
}

impl Commandable for Command {
  #[named]
  fn get_action(&self) -> Action {
    trace_enter!();
    use Command::*;
    let result = match self {
      Look(command) => command.get_action(),
      MoveCompassDirection(command) => command.get_action(),
    };
    trace_exit!();
    result
  }
}
