use specs::prelude::*;

pub trait Actionable {
  /// Checks whether the specified entity should (by its own logic) perform the action.
  fn should_perform(&self, _ecs: &mut World) -> bool {
    true
  }

  /// Checks whether the specified entity can (by game rules) perform the action.
  fn can_perform(&self, _ecs: &mut World) -> bool {
    true
  }

  /// Actually perform the action.
  fn perform(&self, _ecs: &mut World) {}
}
