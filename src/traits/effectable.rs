use specs::prelude::*;

pub trait Effectable {
  /// Execute this effect.
  fn execute(&self, _ecs: &mut World) {
    todo!();
  }
}
