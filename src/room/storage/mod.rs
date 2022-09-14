use crate::component::*;
use super::Room;

zcomponents_storage!(RoomStorage<Room>: {
  has_name: HasName,
  has_description: HasDescription,
  has_exits: HasExits,
});
