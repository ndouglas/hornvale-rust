use specs::prelude::*;

use crate::ecs::resources::RunMode;

use crate::traits::Commandable;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct QuitCommand {
  pub entity: Entity,
}

impl Commandable for QuitCommand {
  #[named]
  fn execute(&self, ecs: &mut World) {
    *ecs.write_resource::<RunMode>() = RunMode::Quit;
  }
}
