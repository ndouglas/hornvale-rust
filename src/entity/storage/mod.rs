use crate::component::*;
use super::Entity;

zcomponents_storage!(EntityStorage<Entity>: {
  has_name: HasName,
  has_description: HasDescription,
  is_in_room: IsInRoom,
});
