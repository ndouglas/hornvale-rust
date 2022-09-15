use super::Room;
use crate::component::*;

zcomponents_storage!(RoomStorage<Room>: {
  has_been_visited: HasBeenVisited,
  has_description: HasDescription,
  has_exits: HasExits,
  has_name: HasName,
});
