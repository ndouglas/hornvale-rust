use specs::prelude::*;

use crate::commands::Command;
use crate::ecs::resources::Tick;

pub trait WorldUsable {
  /// Get the player entity.
  fn get_player_entity(&self) -> Entity;

  /// Get the spawn room entity.
  fn get_spawn_room_entity(&self) -> Entity;

  /// Get the room entity where an entity is currently.
  fn get_entity_room_entity(&self, entity: Entity) -> Option<Entity>;

  /// Get the tick resource.
  fn get_tick(&self) -> Tick;

  /// Insert a command component for the specified entity.
  fn insert_command(&mut self, entity: Entity, command: Command);
}
