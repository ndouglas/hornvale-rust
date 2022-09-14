use super::Entity;
use crate::component::*;

zcomponents_storage!(EntityStorage<Entity>: {
  has_name: HasName,
  has_description: HasDescription,
  is_in_room: IsInRoom,
});
