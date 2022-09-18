#[macro_export]
macro_rules! get_name {
  ($system_data: expr, $entity: expr) => {{
    let mut result = None;
    if let Some(HasName(string)) = $system_data.has_name.get($entity) {
      result = Some(string);
    }
    result
  }};
}


#[macro_export]
macro_rules! get_description {
  ($system_data: expr, $entity: expr) => {{
    let mut result = None;
    if let Some(HasDescription(string)) = $system_data.has_description.get($entity) {
      result = Some(string);
    }
    result
  }};
}

#[macro_export]
macro_rules! get_current_room {
  ($system_data: expr, $entity: expr) => {{
    use crate::component::IsInRoom;
    let mut result = None;
    if let Some(is_in_room) = $system_data.is_in_room.get($entity) {
      if let IsInRoom(Some(room)) = is_in_room {
        result = Some(room.to_owned());  
      }
    }
    result
  }};
}
