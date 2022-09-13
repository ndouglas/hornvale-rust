use specs::prelude::*;

use crate::actions::*;
use crate::commands::Command;
use crate::ecs::components::*;
use crate::ecs::resources::RunMode;
use crate::model::Direction;
use crate::queue::enqueue_action;
use crate::queue::enqueue_message;
use crate::traits::Actionable;
use crate::traits::Commandable;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct QuitCommand {
  pub entity: Entity,
}

impl Commandable for QuitCommand {
  #[named]
  fn execute(&self, ecs: &mut World) {
    let mut run_mode = ecs.write_resource::<RunMode>();
    *run_mode = RunMode::Quit;
  }
}
