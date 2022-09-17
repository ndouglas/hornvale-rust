use crate::command::Command;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct CommandEvent {
  pub command: Command,
}
