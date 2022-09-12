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
  fn get_player_entity(&self) -> Entity {
    trace_enter!();
    let result = self.fetch::<Player>().0;
    trace_exit!();
    result
  }

  #[named]
  fn get_spawn_room_entity(&self) -> Entity {
    trace_enter!();
    let result = self.fetch::<SpawnRoom>().0;
    trace_exit!();
    result
  }

  #[named]
  fn get_entity_room_entity(&self, entity: Entity) -> Option<Entity> {
    trace_enter!();
    let is_in_room_storage = self.read_storage::<IsInRoom>();
    let mut result = None;
    if let Some(IsInRoom(room_entity)) = is_in_room_storage.get(entity) {
      result = Some(room_entity.to_owned());
    }
    trace_exit!();
    result
  }

  #[named]
  fn get_entity_name(&self, entity: Entity) -> Option<String> {
    trace_enter!();
    let name_storage = self.read_storage::<HasName>();
    let mut result = None;
    if let Some(HasName(name)) = name_storage.get(entity) {
      result = Some(name.to_owned());
    }
    trace_exit!();
    result
  }

  #[named]
  fn get_entity_description(&self, entity: Entity) -> Option<String> {
    trace_enter!();
    let description_storage = self.read_storage::<HasDescription>();
    let mut result = None;
    if let Some(HasDescription(description)) = description_storage.get(entity) {
      result = Some(description.to_owned());
    }
    trace_exit!();
    result
  }

  #[named]
  fn get_room_entity_exits(&self, entity: Entity) -> Option<Exits> {
    trace_enter!();
    let has_exits_storage = self.read_storage::<HasExits>();
    let mut result = None;
    if let Some(HasExits { exits }) = has_exits_storage.get(entity) {
      result = Some(exits.to_owned());
    }
    trace_exit!();
    result
  }

  #[named]
  fn get_room_entity_exit(&self, entity: Entity, direction: &Direction) -> Option<Exit> {
    trace_enter!();
    let mut result = None;
    if let Some(exits) = self.get_room_entity_exits(entity) {
      if let Some(exit) = exits.get_exit(direction) {
        result = Some(exit.to_owned());
      }
    }
    trace_exit!();
    result
  }

  #[named]
  fn get_tick(&self) -> Tick {
    trace_enter!();
    let result = *self.fetch::<Tick>();
    trace_exit!();
    result
  }

  #[named]
  fn insert_command(&mut self, entity: Entity, command: Command) {
    trace_enter!();
    self
      .write_storage::<HasCommand>()
      .insert(entity, HasCommand(command.clone()))
      .expect(format!("Could not insert command {:?} for entity {:?}", command, entity).as_str());
    trace_exit!();
  }
}
