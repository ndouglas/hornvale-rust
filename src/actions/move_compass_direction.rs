use specs::prelude::*;

use crate::actions::*;
use crate::commands::Command;
use crate::ecs::components::*;
use crate::model::CompassDirection;
use crate::traits::Actionable;
use crate::traits::Commandable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct MoveCompassDirectionAction {
  pub entity: Entity,
  pub compass_direction: CompassDirection,
}

impl Actionable for MoveCompassDirectionAction {
  #[named]
  fn perform(&self, ecs: &mut World) {
    trace_enter!();
    let mut room_entity_opt = None;
    {
      let is_in_room_storage = ecs.read_storage::<IsInRoom>();
      if let Some(is_in_room) = is_in_room_storage.get(self.entity) {
        room_entity_opt = Some(is_in_room.entity);
      }
    }
    match room_entity_opt {
      Some(room_entity) => {
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
      None => {}
    }
    trace_exit!();
  }
}
