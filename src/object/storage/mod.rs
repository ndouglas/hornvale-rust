use super::Object;
use crate::component::*;

zcomponents_storage!(ObjectStorage<Object>: {
  has_name: HasName,
  has_description: HasDescription,
  is_in_room: IsInRoom,
});
