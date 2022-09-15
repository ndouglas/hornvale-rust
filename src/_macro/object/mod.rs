#[macro_export]
macro_rules! create_object {
  ($name: expr, $description: expr) => {{
    use crate::component::*;
    use crate::entity::ENTITIES;
    let mut entities = ENTITIES.lock().unwrap();
    let entity_id = entities.alloc_id();
    entities.has_name.insert(entity_id, HasName($name.into()));
    entities
      .has_description
      .insert(entity_id, HasDescription($description.into()));
    entities.is_in_room.insert(entity_id, IsInRoom(None));
    entity_id
  }};
  ($name: expr, $description: expr, $in_room: expr) => {{
    use crate::component::*;
    use crate::entity::ENTITIES;
    let mut entities = ENTITIES.lock().unwrap();
    let entity_id = entities.alloc_id();
    entities.has_name.insert(entity_id, HasName($name.into()));
    entities
      .has_description
      .insert(entity_id, HasDescription($description.into()));
    entities.is_in_room.insert(entity_id, IsInRoom(Some($in_room)));
    entity_id
  }};
}
