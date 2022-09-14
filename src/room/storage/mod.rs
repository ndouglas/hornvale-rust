use super::Room;
use crate::component::*;

zcomponents_storage!(RoomStorage<Room>: {
  has_name: HasName,
  has_description: HasDescription,
  has_exits: HasExits,
});
