use std::fmt::{self, Display};
use std::slice::Iter;

use super::input::keystroke::Keystroke;

pub mod hotkey_actions;
pub use hotkey_actions::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HotkeyAction {
  Quit,
  Undo,
  Redo,
}

impl HotkeyAction {
  /// All available actions
  pub fn iterator() -> Iter<'static, HotkeyAction> {
    use HotkeyAction::*;
    static HOTKEY_ACTIONS: [HotkeyAction; 3] = [Quit, Undo, Redo];
    HOTKEY_ACTIONS.iter()
  }

  /// List of keystrokes associated to action
  pub fn keystrokes(&self) -> &[Keystroke] {
    use HotkeyAction::*;
    use Keystroke::*;
    match self {
      Quit => &[Ctrl('c'), Ctrl('q')],
      Undo => &[Ctrl('z')],
      Redo => &[Ctrl('r')],
    }
  }
}

/// To display a user friendly short description of action
impl Display for HotkeyAction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    use HotkeyAction::*;
    let str = match self {
      Quit => "Quit",
      Undo => "Undo",
      Redo => "Redo",
    };
    write!(f, "{}", str)
  }
}
