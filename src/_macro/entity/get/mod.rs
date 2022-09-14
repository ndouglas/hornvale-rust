#[macro_export]
macro_rules! get_player {
  () => {{
    use crate::player::PLAYER;
    PLAYER.lock().unwrap().0.unwrap()
  }};
}

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
macro_rules! get_room_name {
  ($room: expr) => {{
    use crate::component::HasName;
    use crate::room::ROOMS;
    let mut result = None;
    let rooms = ROOMS.lock().unwrap();
    if let Some(HasName(name)) = rooms.has_name.get_opt($room) {
      result = Some(name.to_owned());
    }
    result
  }};
}

#[macro_export]
macro_rules! get_room_description {
  ($room: expr) => {{
    use crate::component::HasDescription;
    use crate::room::ROOMS;
    let mut result = None;
    let rooms = ROOMS.lock().unwrap();
    if let Some(HasDescription(description)) = rooms.has_description.get_opt($room) {
      result = Some(description.to_owned());
    }
    result
  }};
}

#[macro_export]
macro_rules! get_exits {
  ($room: expr) => {{
    use crate::component::HasExits;
    use crate::room::ROOMS;
    let mut result = None;
    let rooms = ROOMS.lock().unwrap();
    if let Some(HasExits { exits }) = rooms.has_exits.get_opt($room) {
      result = Some(exits.to_owned());
    }
    result
  }};
}

#[macro_export]
macro_rules! get_exit_to {
  ($room: expr, $direction: expr) => {{
    let mut result = None;
    if let Some(exits) = get_exits!($room) {
      if let Some(exit) = exits.get_exit($direction) {
        result = Some(exit.to_owned());
      }
    }
    result
  }};
}
