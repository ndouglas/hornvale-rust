use super::super::Eventable;
use crate::action::Action;
use crate::entity::Entity;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum ActionEventType {
  CouldNotPerformAction,
  WillAttemptToPerformAction,
  WillFailToPerformAction,
  DidPerformAction,
  DidFailToPerformAction,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct ActionEvent {
  pub r#type: ActionEventType,
  pub action: Action,
  pub message: Option<String>,
  pub room: Entity,
}

impl Eventable for ActionEvent {
  /// Dispatch this event.
  fn dispatch(&self) {
    use ActionEventType::*;
    match self.r#type {
      CouldNotPerformAction => dispatch_action_event!(self, could_not_perform),
      WillAttemptToPerformAction => dispatch_action_event!(self, will_attempt_to_perform),
      WillFailToPerformAction => dispatch_action_event!(self, will_fail_to_perform),
      DidPerformAction => dispatch_action_event!(self, did_perform),
      DidFailToPerformAction => dispatch_action_event!(self, did_fail_to_perform),
    }
  }
}
