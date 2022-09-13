use specs::prelude::*;
use std::collections::HashMap;

use crate::commands::Command;
use crate::ecs::components::HasCommand;
use crate::ecs::components::HasDescription;
use crate::ecs::components::HasExits;
use crate::ecs::components::HasName;
use crate::ecs::components::IsInRoom;
use crate::ecs::resources::Player;
use crate::ecs::resources::SpawnRoom;
use crate::ecs::resources::Tick;
use crate::model::Direction;
use crate::model::Exit;
use crate::model::Exits;
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
