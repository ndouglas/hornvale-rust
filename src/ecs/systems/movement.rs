use specs::prelude::*;

use crate::ecs::components::*;
use crate::model::CompassDirection;

pub struct Movement {}

impl<'a> System<'a> for Movement {
  type SystemData = (
    Entities<'a>,
    WriteStorage<'a, WantsToMove>,
    WriteStorage<'a, IsInRoom>,
    ReadStorage<'a, IsARoom>,
    ReadStorage<'a, IsAPlayer>,
    ReadStorage<'a, HasRoomExits>,
  );

  fn run(&mut self, data: Self::SystemData) {
    let (entities, mut wants_to_move_storage, mut is_in_room_storage, is_a_room_storage, is_a_player_storage, has_room_exits_storage) = data;
    let mut entities_to_move: Vec<(Entity, Entity)> = Vec::new();
    let mut entities_attempted_move: Vec<Entity> = Vec::new();
    {
      for (entity, wants_to_move, is_in_room) in (&entities, &mut wants_to_move_storage, &mut is_in_room_storage).join() {
        let room_entity = &mut is_in_room.entity;
        if let Some(exits) = &has_room_exits_storage.get(*room_entity) {
          let mut found = false;
          for room_exit in exits.room_exits.iter() {
            if room_exit.compass_direction == wants_to_move.compass_direction {
              let new_room_entity = room_exit.room_entity;
              entities_to_move.push((entity, new_room_entity));
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
        entities_attempted_move.push(entity);
      }
    }
    {
      for entity in entities_attempted_move.iter() {
        wants_to_move_storage.remove(*entity);
      }
    }
    for (entity, room_entity) in entities_to_move.iter() {
      is_in_room_storage
        .insert(*entity, IsInRoom { entity: *room_entity })
        .expect("Unable to insert entity in new room!");
    }
  }
}
