use specs::prelude::*;

pub mod player;
pub use player::*;
pub mod spawn_room;
pub use spawn_room::*;
pub mod tick;
pub use tick::*;

use crate::ecs::components::*;

#[named]
pub fn insert_resources(ecs: &mut World) {
  trace_enter!();
  ecs.insert(Tick(0));
  let room = ecs
    .create_entity()
    .with(HasName { name: "Test Room".into() })
    .with(HasDescription { description: "This is just a nondescript room.".into() })
    .with(IsARoom {})
    .build();
  ecs.insert(SpawnRoom(room));
  let player = ecs
    .create_entity()
    .with(HasName { name: "Player".into() })
    .with(IsAPlayer {})
    .with(IsInRoom { entity: room })
    .build();
  ecs.insert(Player(player));
  trace_exit!();
}
