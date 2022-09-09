use specs::prelude::*;

use crate::actions::*;
use crate::commands::Command;
use crate::ecs::components::*;
use crate::model::CompassDirection;
use crate::traits::Actionable;
use crate::traits::Commandable;
use crate::traits::WorldUsable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct MoveCompassDirectionAction {
  pub entity: Entity,
  pub compass_direction: CompassDirection,
}

impl Actionable for MoveCompassDirectionAction {
  #[named]
  fn perform(&self, ecs: &mut World) {
    trace_enter!();
    if let Some(room_entity) = ecs.get_entity_room_entity(self.entity) {
      let mut is_in_room_storage = ecs.write_storage::<IsInRoom>();
      let has_room_exits_storage = ecs.read_storage::<HasRoomExits>();
      if let Some(exits) = has_room_exits_storage.get(room_entity) {
        let mut found = false;
        for room_exit in exits.room_exits.iter() {
          if room_exit.compass_direction == self.compass_direction {
            let new_room_entity = room_exit.room_entity;
            is_in_room_storage
              .insert(
                self.entity,
                IsInRoom {
                  entity: new_room_entity,
                },
              )
              .expect("Unable to insert entity in new room!");
            found = true;
            break;
          }
        }
        if !found {
          print!("Somebody is unable to move in that direction!\n");
        }
      } else {
        print!("Somebody is unable to move in that direction!\n");
      }
    }
    trace_exit!();
  }
}
