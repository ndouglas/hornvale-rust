use specs::prelude::*;

use crate::traits::Eventable;

pub mod could_not_perform;
pub use could_not_perform::*;
pub mod will_perform;
pub use will_perform::*;
pub mod will_fail_to_perform;
pub use will_fail_to_perform::*;
pub mod did_perform;
pub use did_perform::*;
pub mod did_fail_to_perform;
pub use did_fail_to_perform::*;

pub enum ActionEvent {
  /// When the entity cannot (by game rules) perform the action.
  CouldNotPerform(CouldNotPerformActionEvent),
  /// When the entity will perform the action.
  WillPerform(WillPerformActionEvent),
  /// When the entity will fail to successfully perform the action.
  WillFailToPerform(WillFailToPerformActionEvent),
  /// When the entity successfully performed the action.
  DidPerform(DidPerformActionEvent),
  /// When the entity failed to successfully perform the action.
  DidFailToPerform(DidFailToPerformActionEvent),
}

impl Eventable for ActionEvent {
  /// Dispatch this event.
  fn dispatch(&self, ecs: &mut World) {
    use ActionEvent::*;
    match self {
      CouldNotPerform(action_event) => action_event.dispatch(ecs),
      WillPerform(action_event) => action_event.dispatch(ecs),
      WillFailToPerform(action_event) => action_event.dispatch(ecs),
      DidPerform(action_event) => action_event.dispatch(ecs),
      DidFailToPerform(action_event) => action_event.dispatch(ecs),
    }
  }
}
