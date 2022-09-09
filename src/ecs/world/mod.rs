use specs::prelude::*;

use crate::commands::Command;
use crate::ecs::components::HasCommand;
use crate::ecs::resources::Player;
use crate::ecs::resources::SpawnRoom;
use crate::ecs::resources::Tick;
use crate::traits::WorldUsable;

pub mod room;
pub use room::*;

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
  fn get_tick(&self) -> Tick {
    trace_enter!();
    let result = *self.fetch::<Tick>();
    trace_exit!();
    result
  }

  #[named]
  fn insert_command(&mut self, entity: Entity, command: Command) {
    trace_enter!();
    self.write_storage::<HasCommand>()
      .insert(entity, HasCommand { command })
      .expect(format!("Could not insert command {:?} for entity {:?}", command, entity).as_str());
    trace_exit!();
  }



}