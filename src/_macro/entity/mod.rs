#[macro_export]
macro_rules! get_current_room {
  ($entity: expr) => {{
    use crate::component::IsInRoom;
    use crate::entity::ENTITIES;
    let mut result = None;
    let entities = ENTITIES.lock().unwrap();
    if let Some(IsInRoom(room)) = entities.is_in_room.get_opt($entity) {
      result = Some(room.to_owned());
    }
    result
  }};
}

#[macro_export]
macro_rules! move_entity {
  ($entity: expr, $room: expr) => {{
    use crate::entity::ENTITIES;
    let mut entities = ENTITIES.lock().unwrap();
    if let Some(is_in_room) = entities.is_in_room.get_opt_mut($entity) {
      is_in_room.0 = $room;
    }
  }};
}
