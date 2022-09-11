use specs::prelude::*;
use std::collections::HashMap;

pub mod player;
pub use player::*;
pub mod spawn_room;
pub use spawn_room::*;
pub mod tick;
pub use tick::*;

use crate::ecs::components::*;
use crate::model::Direction;
use crate::model::RoomExit;

#[named]
pub fn create_room(ecs: &mut World, name: String) -> Entity {
  trace_enter!();
  let entity = ecs
    .create_entity()
    .with(HasName(name))
    .with(HasDescription("This is just a nondescript room.".into()))
    .with(IsARoom {})
    .build();
  trace_exit!();
  entity
}

#[named]
pub fn insert_resources(ecs: &mut World) {
  trace_enter!();
  ecs.insert(Tick(0));
  let spawn_room = create_room(ecs, "Spawn Room".into());
  ecs.insert(SpawnRoom(spawn_room));
  let ne_room = create_room(ecs, "Northeast Room".into());
  let n_room = create_room(ecs, "North Room".into());
  let nw_room = create_room(ecs, "Northwest Room".into());
  let e_room = create_room(ecs, "East Room".into());
  let w_room = create_room(ecs, "West Room".into());
  let se_room = create_room(ecs, "Southeast Room".into());
  let s_room = create_room(ecs, "South Room".into());
  let sw_room = create_room(ecs, "Southwest Room".into());
  let player = ecs
    .create_entity()
    .with(HasName("Player".into()))
    .with(IsAPlayer {})
    .with(IsInRoom(spawn_room))
    .build();
  ecs.insert(Player(player));
  {
    let mut has_room_exit_storage = ecs.write_storage::<HasRoomExits>();
    has_room_exit_storage
      .insert(
        spawn_room,
        HasRoomExits(HashMap::from([(
          Direction::Northeast,
          RoomExit {
            direction: Direction::Northeast,
            room_entity: ne_room,
            is_passable: true,
          },
        )])),
      )
      .expect("Unable to insert exit.");
    has_room_exit_storage
      .insert(
        ne_room,
        HasRoomExits(HashMap::from([(
          Direction::Southwest,
          RoomExit {
            direction: Direction::Southwest,
            room_entity: spawn_room,
            is_passable: true,
          },
        )])),
      )
      .expect("Unable to insert exit.");
  }

  trace_exit!();
}
