use specs::prelude::*;

use crate::actions::Action;
use crate::traits::Commandable;

pub mod echo;
pub use echo::*;
pub mod look;
pub use look::*;
pub mod move_compass_direction;
pub use move_compass_direction::*;

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Command {
  Echo(EchoCommand),
  Look(LookCommand),
  MoveDirection(MoveDirectionCommand),
}

impl Commandable for Command {
  #[named]
  fn execute(&self, ecs: &mut World) {
    trace_enter!();
    use Command::*;
    match self {
      Echo(command) => command.execute(ecs),
      Look(command) => command.execute(ecs),
      MoveDirection(command) => command.execute(ecs),
    }
    trace_exit!();
  }
}
