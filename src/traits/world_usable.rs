use specs::prelude::*;
use std::collections::HashMap;

use crate::commands::Command;
use crate::ecs::resources::Tick;
use crate::model::Direction;
use crate::model::Exit;
use crate::model::Exits;

pub trait WorldUsable {
  /// Insert a command component for the specified entity.
  fn insert_command(&mut self, entity: Entity, command: Command);
}
