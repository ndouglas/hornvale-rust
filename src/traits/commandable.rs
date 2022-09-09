use specs::prelude::*;

pub trait Commandable {
  /// Execute this command.
  fn execute(&self, ecs: &mut World);
}
