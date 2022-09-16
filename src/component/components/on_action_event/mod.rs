use crate::entity::Entity;
use crate::event::ActionEvent;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct OnActionEvent {
  pub could_not_perform: Option<fn(ActionEvent)>,
  pub will_attempt_to_perform: Option<fn(ActionEvent)>,
  pub will_fail_to_perform: Option<fn(ActionEvent)>,
  pub did_perform: Option<fn(ActionEvent)>,
  pub did_fail_to_perform: Option<fn(ActionEvent)>,
}