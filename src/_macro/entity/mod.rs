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
      is_in_room.0 = Some($room);
    }
  }};
}

#[macro_export]
macro_rules! get_name {
  ($entity: expr) => {{
    use crate::component::*;
    use crate::entity::ENTITIES;
    let mut result = None;
    let entities = ENTITIES.lock().unwrap();
    if let Some(HasName(name)) = entities.has_name.get_opt($entity) {
      result = Some(name.to_owned());
    }
    if let Some(OnGetName(get_name)) = entities.on_get_name.get_opt($entity) {
      if let Some(name) = get_name($entity) {
        result = Some(name);
      }
    }
    result
  }};
}

#[macro_export]
macro_rules! get_description {
  ($entity: expr) => {{
    use crate::component::*;
    use crate::entity::ENTITIES;
    let mut result = None;
    let entities = ENTITIES.lock().unwrap();
    if let Some(HasDescription(description)) = entities.has_description.get_opt($entity) {
      result = Some(description.to_owned());
    }
    if let Some(OnGetDescription(get_description)) = entities.on_get_description.get_opt($entity) {
      if let Some(description) = get_description($entity) {
        result = Some(description);
      }
    }
    result
  }};
}

#[macro_export]
macro_rules! add_component {
  ($entity: expr, $type: ident, $value: expr) => {{
    use crate::entity::ENTITIES;
    use crate::component::*;
    let mut entities = ENTITIES.lock().unwrap();
    entities.$type.insert($entity, $value);
  }};
}
