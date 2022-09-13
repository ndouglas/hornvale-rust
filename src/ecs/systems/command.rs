use specs::prelude::*;

use crate::ecs::components::*;

pub struct CommandSystem {}

impl<'a> System<'a> for CommandSystem {
  type SystemData = (Entities<'a>, WriteStorage<'a, HasCommand>);

  #[named]
  fn run(&mut self, data: Self::SystemData) {
    let (entities, mut has_command_storage) = data;
    let mut entities_commanded: Vec<Entity> = Vec::new();
    {
      for (entity, has_command) in (&entities, &mut has_command_storage).join() {
        entities_commanded.push(entity);
        enq_command!(has_command.0.clone());
      }
    }
    {
      for entity in entities_commanded.iter() {
        has_command_storage.remove(*entity);
      }
    }
  }
}
