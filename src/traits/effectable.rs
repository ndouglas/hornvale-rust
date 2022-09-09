use specs::prelude::*;

pub trait Effectable {
  /// Execute this effect.
  fn execute(&self, ecs: &mut World);
}
