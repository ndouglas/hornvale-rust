use super::key::Key;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum InputEvent {
  /// An input event occurred.
  Input(Key),
  /// An tick event occurred.
  Tick,
}