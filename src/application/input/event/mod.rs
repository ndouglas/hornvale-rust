use super::keystroke::Keystroke;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum InputEvent {
  Keystroke(Keystroke),
  Tick,
}
