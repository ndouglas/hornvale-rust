#[macro_export]
macro_rules! move_entity_room {
  ($entity: expr, $room: expr) => {{
    use crate::component::IsInRoom;
    use crate::entity::ENTITIES;
    let mut entities = ENTITIES.lock().unwrap();
    if let Some(is_in_room) = entities.is_in_room.get_opt_mut($entity) {
      is_in_room.0 = $room;
    }
  }};
}
