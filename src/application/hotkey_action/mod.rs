use std::fmt::{self, Display};
use std::slice::Iter;

use super::input::keystroke::Keystroke;

pub mod hotkey_actions;
pub use hotkey_actions::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HotkeyAction {
  Quit,
  Sleep,
}

impl HotkeyAction {
  /// All available actions
  pub fn iterator() -> Iter<'static, HotkeyAction> {
    use HotkeyAction::*;
    static HOTKEY_ACTIONS: [HotkeyAction; 2] = [Quit, Sleep];
    HOTKEY_ACTIONS.iter()
  }

  /// List of keystrokes associated to action
  pub fn keystrokes(&self) -> &[Keystroke] {
    use HotkeyAction::*;
    use Keystroke::*;
    match self {
      Quit => &[Ctrl('c'), Ctrl('q')],
      Sleep => &[Ctrl('s')],
    }
  }
}

/// To display a user friendly short description of action
impl Display for HotkeyAction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use HotkeyAction::*;
    let str = match self {
      Quit => "Quit",
      Sleep => "Sleep",
    };
    write!(f, "{}", str)
  }
}
