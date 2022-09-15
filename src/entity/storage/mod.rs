use super::Entity;
use crate::component::*;

zcomponents_storage!(EntityStorage<Entity>: {
  has_description: HasDescription,
  has_exits: HasExits,
  has_name: HasName,
  has_visited_rooms: HasVisitedRooms,
  is_in_room: IsInRoom,
  on_get_description: OnGetDescription,
  on_get_name: OnGetName,
});
