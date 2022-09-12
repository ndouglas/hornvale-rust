use specs::prelude::*;
use std::collections::HashMap;

use crate::commands::Command;
use crate::ecs::resources::Tick;
use crate::model::Direction;
use crate::model::Exit;
use crate::model::Exits;

pub trait WorldUsable {
  /// Get the player entity.
  fn get_player_entity(&self) -> Entity;

  /// Get the spawn room entity.
  fn get_spawn_room_entity(&self) -> Entity;

  /// Get the room entity where an entity is currently.
  fn get_entity_room_entity(&self, entity: Entity) -> Option<Entity>;

  /// Get entity name.
  fn get_entity_name(&self, entity: Entity) -> Option<String>;

  /// Get entity description.
  fn get_entity_description(&self, entity: Entity) -> Option<String>;

  /// Get room entity exits.
  fn get_room_entity_exits(&self, entity: Entity) -> Option<Exits>;

  /// Get room entity exit to a particular direction.
  fn get_room_entity_exit(&self, entity: Entity, direction: &Direction) -> Option<Exit>;

  /// Get the tick resource.
  fn get_tick(&self) -> Tick;

  /// Insert a command component for the specified entity.
  fn insert_command(&mut self, entity: Entity, command: Command);
}
