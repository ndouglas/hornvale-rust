use specs::prelude::*;

pub trait Actionable {
  /// Actually attempt the action.
  fn attempt(&self, _ecs: &mut World) {
    todo!();
  }
}
