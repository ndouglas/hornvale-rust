
use std::fmt::{self, Display};
use std::slice::Iter;

use super::input::keystroke::Keystroke;

pub mod actions;
pub use actions::*;

/// We define all available action
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Action {
  Quit,
  Sleep,
}

impl Action {
  /// All available actions
  pub fn iterator() -> Iter<'static, Action> {
    static ACTIONS: [Action; 2] = [Action::Quit, Action::Sleep];
    ACTIONS.iter()
  }

  /// List of keystrokes associated to action
  pub fn keystrokes(&self) -> &[Keystroke] {
    match self {
      Action::Quit => &[Keystroke::Ctrl('c'), Keystroke::Ctrl('q')],
      Action::Sleep => &[Keystroke::Ctrl('s')],
    }
  }
}

/// To display a user friendly short description of action
impl Display for Action {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let str = match self {
      Action::Quit => "Quit",
      Action::Sleep => "Sleep",
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
}
