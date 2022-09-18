#[macro_export]
macro_rules! create_object {
  ($system_data: expr, $name: expr, $description: expr) => {{
    use crate::component::*;
    let object_id = $system_data.entities.create();
    $system_data.has_name.insert(object_id, HasName($name.into()));
    $system_data
      .has_description
      .insert(object_id, HasDescription($description.into()));
    $system_data.is_an_object.insert(object_id, IsAnObject);
    $system_data.is_in_room.insert(object_id, IsInRoom(None));
    object_id
  }};
  ($system_data: expr, $name: expr, $description: expr, $in_room: expr) => {{
    use crate::component::*;
    let object_id = $system_data.entities.create();
    $system_data.has_name.insert(object_id, HasName($name.into()));
    $system_data
      .has_description
      .insert(object_id, HasDescription($description.into()));
    $system_data.is_an_object.insert(object_id, IsAnObject);
    $system_data.is_in_room.insert(object_id, IsInRoom(Some($in_room)));
    object_id
  }};
}
