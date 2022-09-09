use specs::prelude::*;

pub mod has_command;
pub use has_command::*;
pub mod has_description;
pub use has_description::*;
pub mod has_name;
pub use has_name::*;
pub mod has_room_exits;
pub use has_room_exits::*;
pub mod is_a_player;
pub use is_a_player::*;
pub mod is_a_room;
pub use is_a_room::*;
pub mod is_in_room;
pub use is_in_room::*;

#[named]
pub fn register_components(ecs: &mut World) {
  trace_enter!();
  ecs.register::<HasCommand>();
  ecs.register::<HasDescription>();
  ecs.register::<HasName>();
  ecs.register::<HasRoomExits>();
  ecs.register::<IsAPlayer>();
  ecs.register::<IsARoom>();
  ecs.register::<IsInRoom>();
  trace_exit!();
}
