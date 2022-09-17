use super::key::Key;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum InputEvent {
  Input(Key),
  Tick,
}
