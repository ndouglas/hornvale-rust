use std::fmt::{self, Display, Formatter};

use crossterm::event;
use tui_textarea::{Input, Key};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Keystroke {
  /// Both Enter (or Return) and numpad Enter
  Enter,
  Tab,
  Backspace,
  Esc,

  Left,
  Right,
  Up,
  Down,

  Insert,
  Delete,
  Home,
  End,
  PageUp,
  PageDown,

  F0,
  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,
  Char(char),
  Ctrl(char),
  Alt(char),
  Shift(char),
  Unknown,
}

impl Keystroke {
  pub fn get_ctrl(&self) -> bool {
    if let Keystroke::Ctrl(_char) = self {
      true
    } else {
      false
    }
  }

  pub fn get_alt(&self) -> bool {
    if let Keystroke::Alt(_char) = self {
      true
    } else {
      false
    }
  }

  pub fn get_shift(&self) -> bool {
    if let Keystroke::Shift(_char) = self {
      true
    } else {
      false
    }
  }

  /// Returns the function key corresponding to the given number
  ///
  /// 1 -> F1, etc...
  ///
  /// # Panics
  ///
  /// If `n == 0 || n > 12`
  pub fn from_f(n: u8) -> Self {
    use Keystroke::*;
    match n {
      0 => F0,
      1 => F1,
      2 => F2,
      3 => F3,
      4 => F4,
      5 => F5,
      6 => F6,
      7 => F7,
      8 => F8,
      9 => F9,
      10 => F10,
      11 => F11,
      12 => F12,
      _ => panic!("unknown function key: F{}", n),
    }
  }
}

impl Display for Keystroke {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    use Keystroke::*;
    match *self {
      Alt(' ') => write!(f, "<Alt+Space>"),
      Ctrl(' ') => write!(f, "<Ctrl+Space>"),
      Char(' ') => write!(f, "<Space>"),
      Alt(char) => write!(f, "<Alt+{}>", char),
      Ctrl(char) => write!(f, "<Ctrl+{}>", char),
      Char(char) => write!(f, "<{}>", char),
      _ => write!(f, "<{:?}>", self),
    }
  }
}

impl From<event::KeyEvent> for Keystroke {
  fn from(key_event: event::KeyEvent) -> Self {
    use Keystroke::*;
    match key_event {
      event::KeyEvent {
        code: event::KeyCode::Esc,
        ..
      } => Esc,
      event::KeyEvent {
        code: event::KeyCode::Backspace,
        ..
      } => Backspace,
      event::KeyEvent {
        code: event::KeyCode::Left,
        ..
      } => Left,
      event::KeyEvent {
        code: event::KeyCode::Right,
        ..
      } => Right,
      event::KeyEvent {
        code: event::KeyCode::Up,
        ..
      } => Up,
      event::KeyEvent {
        code: event::KeyCode::Down,
        ..
      } => Down,
      event::KeyEvent {
        code: event::KeyCode::Home,
        ..
      } => Home,
      event::KeyEvent {
        code: event::KeyCode::End,
        ..
      } => End,
      event::KeyEvent {
        code: event::KeyCode::PageUp,
        ..
      } => PageUp,
      event::KeyEvent {
        code: event::KeyCode::PageDown,
        ..
      } => PageDown,
      event::KeyEvent {
        code: event::KeyCode::Delete,
        ..
      } => Delete,
      event::KeyEvent {
        code: event::KeyCode::Insert,
        ..
      } => Insert,
      event::KeyEvent {
        code: event::KeyCode::F(n),
        ..
      } => Keystroke::from_f(n),
      event::KeyEvent {
        code: event::KeyCode::Enter,
        ..
      } => Enter,
      event::KeyEvent {
        code: event::KeyCode::Tab,
        ..
      } => Tab,

      // First check for char + modifier
      event::KeyEvent {
        code: event::KeyCode::Char(char),
        modifiers: event::KeyModifiers::ALT,
        ..
      } => Alt(char),
      event::KeyEvent {
        code: event::KeyCode::Char(char),
        modifiers: event::KeyModifiers::CONTROL,
        ..
      } => Ctrl(char),
      event::KeyEvent {
        code: event::KeyCode::Char(char),
        modifiers: event::KeyModifiers::SHIFT,
        ..
      } => Shift(char),
      event::KeyEvent {
        code: event::KeyCode::Char(char),
        ..
      } => Char(char),

      _ => Unknown,
    }
  }
}

impl From<Keystroke> for Input {
  fn from(keystroke: Keystroke) -> Self {
    let ctrl = keystroke.get_ctrl();
    let alt = keystroke.get_alt();
    use Keystroke::*;
    let key = match keystroke {
      Char(c) => Key::Char(c),
      Shift(c) => Key::Char(c),
      Backspace => Key::Backspace,
      Enter => Key::Enter,
      Left => Key::Left,
      Right => Key::Right,
      Up => Key::Up,
      Down => Key::Down,
      Tab => Key::Tab,
      Delete => Key::Delete,
      Home => Key::Home,
      End => Key::End,
      PageUp => Key::PageUp,
      PageDown => Key::PageDown,
      Esc => Key::Esc,
      F0 => Key::F(0),
      F1 => Key::F(1),
      F2 => Key::F(2),
      F3 => Key::F(3),
      F4 => Key::F(4),
      F5 => Key::F(5),
      F6 => Key::F(6),
      F7 => Key::F(7),
      F8 => Key::F(8),
      F9 => Key::F(9),
      F10 => Key::F(10),
      F11 => Key::F(11),
      F12 => Key::F(12),
      _ => Key::Null,
    };
    Self { key, ctrl, alt }
  }
}
