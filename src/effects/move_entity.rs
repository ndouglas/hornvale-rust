use specs::prelude::*;

use crate::ecs::components::*;
use crate::effects::Effect;
use crate::effects::LookEffect;

use crate::queue::enqueue_effect;

use crate::traits::Effectable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct MoveEntityEffect {
  pub entity: Entity,
  pub from: Entity,
  pub to: Entity,
}

impl Effectable for MoveEntityEffect {
  #[named]
  fn execute(&self, ecs: &mut World) {
    move_entity_room!(ecs, self.entity, self.to);
    enq_effect!(Effect::Look(LookEffect { entity: self.entity }));
  }
}
