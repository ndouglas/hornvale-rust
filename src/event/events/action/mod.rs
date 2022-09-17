use crate::action::Action;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct ActionEvent {
  pub action: Action,
}
