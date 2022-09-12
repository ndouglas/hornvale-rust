use colored::*;
use specs::prelude::*;

use crate::actions::*;
use crate::commands::Command;
use crate::ecs::components::*;
use crate::model::Direction;
use crate::queue::enqueue_message;
use crate::traits::Commandable;
use crate::traits::Effectable;
use crate::traits::WorldUsable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct LookEffect {
  pub entity: Entity,
}

impl Effectable for LookEffect {
  #[named]
  fn execute(&self, ecs: &mut World) {
    if let Some(room) = get_current_room!(ecs, self.entity) {
      enq_message!(format_room!(ecs, room));
    }
  }
}
