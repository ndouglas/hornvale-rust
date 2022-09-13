use specs::prelude::*;

pub trait Commandable {
  /// Execute this command.
  fn execute(&self, _ecs: &mut World) {
    todo!();
  }
}
