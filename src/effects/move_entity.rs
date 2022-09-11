use specs::prelude::*;

use crate::actions::*;
use crate::commands::Command;
use crate::ecs::components::*;
use crate::effects::Effect;
use crate::effects::LookEffect;
use crate::model::Direction;
use crate::queue::enqueue_effect;
use crate::traits::Commandable;
use crate::traits::Effectable;
use crate::traits::WorldUsable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct MoveEntityEffect {
  pub entity: Entity,
  pub from: Entity,
  pub to: Entity,
}

impl Effectable for MoveEntityEffect {
  #[named]
  fn execute(&self, ecs: &mut World) {
    trace_enter!();
    let mut is_in_room_storage = ecs.write_storage::<IsInRoom>();
    is_in_room_storage
      .insert(self.entity, IsInRoom(self.to))
      .expect("Unable to insert entity in new room!");
    enqueue_effect(Effect::Look(LookEffect { entity: self.entity }));
    trace_exit!();
  }
}
