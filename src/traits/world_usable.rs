use specs::prelude::*;

use crate::commands::Command;

pub trait WorldUsable {
  /// Insert a command component for the specified entity.
  fn insert_command(&mut self, entity: Entity, command: Command);
}
