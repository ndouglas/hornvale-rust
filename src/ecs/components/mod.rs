use specs::prelude::*;

pub mod has_name;
pub use has_name::*;
pub mod is_a_player;
pub use is_a_player::*;
pub mod is_a_room;
pub use is_a_room::*;
pub mod is_in_room;
pub use is_in_room::*;

pub fn register_components(ecs: &mut World) {
  ecs.register::<HasName>();
  ecs.register::<IsAPlayer>();
  ecs.register::<IsARoom>();
  ecs.register::<IsInRoom>();
}
