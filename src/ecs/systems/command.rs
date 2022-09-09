use specs::prelude::*;

use crate::actions::Action;
use crate::commands::Command as CommandEnum;
use crate::ecs::components::*;
use crate::model::CompassDirection;
use crate::traits::commandable::Commandable;

pub struct Command {}

impl<'a> System<'a> for Command {
  type SystemData = (Entities<'a>, WriteStorage<'a, HasCommand>, WriteStorage<'a, HasAction>);

  fn run(&mut self, data: Self::SystemData) {
    let (entities, mut has_command_storage, mut has_action_storage) = data;
    let mut entities_commanded: Vec<Entity> = Vec::new();
    {
      for (entity, has_command) in (&entities, &mut has_command_storage).join() {
        entities_commanded.push(entity);
        has_action_storage
          .insert(
            entity,
            HasAction {
              action: has_command.command.get_action(),
            },
          )
          .expect("Unable to insert action for entity!");
      }
    }
    {
      for entity in entities_commanded.iter() {
        has_command_storage.remove(*entity);
      }
    }
  }
}
