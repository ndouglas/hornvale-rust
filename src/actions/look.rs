use specs::prelude::*;

use crate::actions::*;
use crate::commands::Command;
use crate::ecs::components::*;
use crate::model::CompassDirection;
use crate::traits::Actionable;
use crate::traits::Commandable;
use crate::traits::WorldUsable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct LookAction {
  pub entity: Entity,
}

impl Actionable for LookAction {
  #[named]
  fn perform(&self, ecs: &mut World) {
    trace_enter!();
    let mut room_entity_opt = ecs.get_entity_room_entity(self.entity);
    match room_entity_opt {
      Some(room_entity) => {
        let has_name_storage = ecs.read_storage::<HasName>();
        let has_description_storage = ecs.read_storage::<HasDescription>();
        if let Some(name) = has_name_storage.get(room_entity) {
          print!("{}\n", name.name);
        }
        if let Some(description) = has_description_storage.get(room_entity) {
          print!("{}\n", description.description);
        }
      }
      None => {}
    }
    trace_exit!();
  }
}
