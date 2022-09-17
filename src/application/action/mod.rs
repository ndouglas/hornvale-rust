use std::collections::HashMap;
use std::fmt::{self, Display};
use std::slice::Iter;

use super::input::key::Key;

pub mod actions;
pub use actions::*;

/// We define all available action
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Action {
  Quit,
  Sleep,
  IncrementDelay,
  DecrementDelay,
}

impl Action {
  /// All available actions
  pub fn iterator() -> Iter<'static, Action> {
    static ACTIONS: [Action; 4] = [
      Action::Quit,
      Action::Sleep,
      Action::IncrementDelay,
      Action::DecrementDelay,
    ];
    ACTIONS.iter()
  }

  /// List of key associated to action
  pub fn keys(&self) -> &[Key] {
    match self {
      Action::Quit => &[Key::Ctrl('c'), Key::Ctrl('q')],
      Action::Sleep => &[Key::Ctrl('s')],
      Action::IncrementDelay => &[Key::Ctrl('+')],
      Action::DecrementDelay => &[Key::Ctrl('-')],
    }
  }
}

/// Could display a user friendly short description of action
impl Display for Action {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let str = match self {
      Action::Quit => "Quit",
      Action::Sleep => "Sleep",
      Action::IncrementDelay => "Increment delay",
      Action::DecrementDelay => "Decrement delay",
    };
    write!(f, "{}", str)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_find_action_by_key() {
    let actions: Actions = vec![Action::Quit, Action::Sleep].into();
    let result = actions.find(Key::Ctrl('c'));
    assert_eq!(result, Some(&Action::Quit));
  }

  #[test]
  fn should_find_action_by_key_not_found() {
    let actions: Actions = vec![Action::Quit, Action::Sleep].into();
    let result = actions.find(Key::Alt('w'));
    assert_eq!(result, None);
  }

  #[test]
  fn should_create_actions_from_vec() {
    let _actions: Actions = vec![
      Action::Quit,
      Action::Sleep,
      Action::IncrementDelay,
      Action::DecrementDelay,
    ]
    .into();
  }

  #[test]
  #[should_panic]
  fn should_panic_when_create_actions_conflict_key() {
    let _actions: Actions = vec![
      Action::Quit,
      Action::DecrementDelay,
      Action::Sleep,
      Action::IncrementDelay,
      Action::IncrementDelay,
      Action::Quit,
      Action::DecrementDelay,
    ]
    .into();
  }
}
