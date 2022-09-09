use specs::prelude::*;

pub trait Actionable {
  /// Checks whether the specified entity should (by its own logic) perform the action.
  fn should_perform(&self, ecs: &mut World) -> bool {
    true
  }

  /// Checks whether the specified entity can (by game rules) perform the action.
  fn can_perform(&self, ecs: &mut World) -> bool {
    true
  }

  /// Actually perform the action.
  fn perform(&self, ecs: &mut World) {}

  // Emits an event when the entity cannot (by game rules) perform the action.
  // pub fn could_not_perform(&self, ecs: &mut World);

  // Emits an event when the entity will perform the action.
  // pub fn will_perform(&self, ecs: &mut World);

  // Emits an event when the entity will fail to successfully perform the action.
  // pub fn will_fail_to_perform(&self, ecs: &mut World);

  // Emits an event when the entity successfully performed the action.
  // pub fn did_perform(&self, ecs: &mut World);

  // Emits an event when the entity failed to successfully perform the action.
  // pub fn did_fail_to_perform(&self, ecs: &mut World);
}
