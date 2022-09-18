use specs::prelude::*;

use crate::component::components::*;

#[named]
pub fn register_components(ecs: &mut World) {
  ecs.register::<HasDescription>();
  ecs.register::<HasExits>();
  ecs.register::<HasName>();
  ecs.register::<IsAnObject>();
  ecs.register::<IsAPlayer>();
  ecs.register::<IsARoom>();
  ecs.register::<IsInRoom>();
}
