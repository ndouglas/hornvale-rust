#[macro_export]
macro_rules! create_object {
  ($name: expr, $description: expr, $in_room: expr) => {{
    use crate::component::{HasDescription, HasName, IsInRoom};
    use crate::entity::ENTITIES;
    let mut entities = ENTITIES.lock().unwrap();
    let player_id = entities.alloc_id();
    entities.has_name.insert(player_id, HasName($name.into()));
    entities
      .has_description
      .insert(player_id, HasDescription($description.into()));
    entities.is_in_room.insert(player_id, IsInRoom($in_room));
    object_id
  }};
}
