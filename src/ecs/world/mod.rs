use specs::prelude::*;

use crate::commands::Command;
use crate::ecs::components::HasCommand;

use crate::traits::WorldUsable;

impl WorldUsable for World {
  #[named]
  fn insert_command(&mut self, entity: Entity, command: Command) {
    self
      .write_storage::<HasCommand>()
      .insert(entity, HasCommand(command.clone()))
      .expect(format!("Could not insert command {:?} for entity {:?}", command, entity).as_str());
  }
}
