use specs::prelude::*;

use crate::actions::Action as ActionEnum;
use crate::ecs::components::*;
use crate::queue::enqueue_action;
use crate::traits::Actionable;

pub struct ActionSystem {}

impl<'a> System<'a> for ActionSystem {
  type SystemData = (Entities<'a>, WriteStorage<'a, HasAction>);

  #[named]
  fn run(&mut self, data: Self::SystemData) {
    let (entities, mut has_action_storage) = data;
    let mut entities_actioned: Vec<Entity> = Vec::new();
    {
      for (entity, has_action) in (&entities, &mut has_action_storage).join() {
        entities_actioned.push(entity);
        enq_action!(has_action.0.clone());
      }
    }
    {
      for entity in entities_actioned.iter() {
        has_action_storage.remove(*entity);
      }
    }
  }
}
