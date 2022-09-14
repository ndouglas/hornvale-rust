#[macro_export]
macro_rules! get_player {
  () => {{
    use crate::player::PLAYER;
    PLAYER.lock().unwrap().0.unwrap()
  }};
}

#[macro_export]
macro_rules! get_current_room {
  ($ecs: expr, $entity: expr) => {{
    use crate::component::IsInRoom;
    let mut result = None;
    if let Some(IsInRoom(room_entity)) = $ecs.read_storage::<IsInRoom>().get($entity) {
      result = Some(room_entity.to_owned());
    }
    result
  }};
}

#[macro_export]
macro_rules! get_name {
  ($ecs: expr, $entity: expr) => {{
    use crate::component::HasName;
    let mut result = None;
    if let Some(HasName(name)) = $ecs.read_storage::<HasName>().get($entity) {
      result = Some(name.to_owned());
    }
    result
  }};
}

#[macro_export]
macro_rules! get_description {
  ($ecs: expr, $entity: expr) => {{
    use crate::component::HasDescription;
    let mut result = None;
    if let Some(HasDescription(description)) = $ecs.read_storage::<HasDescription>().get($entity) {
      result = Some(description.to_owned());
    }
    result
  }};
}

#[macro_export]
macro_rules! get_exits {
  ($ecs: expr, $entity: expr) => {{
    use crate::component::HasExits;
    let mut result = None;
    if let Some(HasExits { exits }) = $ecs.read_storage::<HasExits>().get($entity) {
      result = Some(exits.to_owned());
    }
    result
  }};
}

#[macro_export]
macro_rules! get_exit_to {
  ($ecs: expr, $entity: expr, $direction: expr) => {{
    let mut result = None;
    if let Some(exits) = get_exits!($ecs, $entity) {
      if let Some(exit) = exits.get_exit($direction) {
        result = Some(exit.to_owned());
      }
    }
    result
  }};
}
