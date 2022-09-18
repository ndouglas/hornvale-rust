use specs::prelude::*;

use crate::component::components::*;

#[named]
pub fn register_components(ecs: &mut World) {
  ecs.register::<HasDescription>();
  ecs.register::<HasName>();
}
