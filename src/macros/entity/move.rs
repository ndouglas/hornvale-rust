#[macro_export]
macro_rules! move_entity_room {
  ($ecs: expr, $entity: expr, $room: expr) => {{
    use crate::ecs::components::IsInRoom;
    $ecs
      .write_storage::<IsInRoom>()
      .insert($entity, IsInRoom($room))
      .expect(&format!("Unable to move entity {:?} to room {:?}!", $entity, $room));
  }};
}
