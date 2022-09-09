use specs::prelude::*;

use crate::commands::*;
use crate::ecs::components::*;
use crate::model::CompassDirection;

pub struct Movement {}

impl<'a> System<'a> for Movement {
  type SystemData = (
    Entities<'a>,
    WriteStorage<'a, HasCommand>,
    WriteStorage<'a, IsInRoom>,
    ReadStorage<'a, IsARoom>,
    ReadStorage<'a, IsAPlayer>,
    ReadStorage<'a, HasRoomExits>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (entities, mut has_command_storage, mut is_in_room_storage, is_a_room_storage, is_a_player_storage, has_room_exits_storage) = data;
    let mut entities_to_move: Vec<(Entity, Entity)> = Vec::new();
    let mut entities_attempted_move: Vec<Entity> = Vec::new();
    {
      for (entity, has_command, is_in_room) in (&entities, &mut has_command_storage, &mut is_in_room_storage)
        .join()
        .filter(|(_entity, has_command, _is_in_room_storage)| match has_command {
          HasCommand { command: Command::MoveCompassDirection(..) } => true,
          _ => false,
        })
        .into_iter()
        .collect::<Vec<_>>()
        .iter() {
        let room_entity = &is_in_room.entity;
        if let Command::MoveCompassDirection(MoveCompassDirection { compass_direction }) = has_command.command {
          if let Some(exits) = &has_room_exits_storage.get(*room_entity) {
            let mut found = false;
            for room_exit in exits.room_exits.iter() {
              if room_exit.compass_direction == compass_direction {
                let new_room_entity = room_exit.room_entity;
                entities_to_move.push((*entity, new_room_entity));
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
          entities_attempted_move.push(*entity);  
        }
      }
    }
    {
      for entity in entities_attempted_move.iter() {
        has_command_storage.remove(*entity);
      }
    }
    for (entity, room_entity) in entities_to_move.iter() {
      is_in_room_storage
        .insert(*entity, IsInRoom { entity: *room_entity })
        .expect("Unable to insert entity in new room!");
    }
  }
}
