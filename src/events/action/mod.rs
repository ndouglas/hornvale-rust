use super::Eventable;
use crate::actions::Action;

pub enum ActionEvent {
  /// When the entity cannot (by game rules) perform the action.
  CouldNotPerformAction { action: Action, message: Option<String> },
  /// When the entity will attempt to perform the action.
  WillAttemptToPerformAction { action: Action, message: Option<String> },
  /// When the entity will fail to successfully perform the action.
  WillFailToPerformAction { action: Action, message: Option<String> },
  /// When the entity successfully performed the action.
  DidPerformAction { action: Action, message: Option<String> },
  /// When the entity failed to successfully perform the action.
  DidFailToPerformAction { action: Action, message: Option<String> },
}

impl Eventable for ActionEvent {
  /// Dispatch this event.
  fn dispatch(&self) {
    use ActionEvent::*;
    match self {
      CouldNotPerformAction { .. } => {},
      WillAttemptToPerformAction { .. } => {},
      WillFailToPerformAction { .. } => {},
      DidPerformAction { .. } => {},
      DidFailToPerformAction { .. } => {},
    }
  }
}
