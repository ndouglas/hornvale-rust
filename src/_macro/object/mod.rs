#[macro_export]
macro_rules! create_object {
  ($name: expr, $description: expr) => {{
    use crate::component::{HasDescription, HasName, IsInRoom};
    use crate::object::OBJECTS;
    let mut objects = OBJECTS.lock().unwrap();
    let obj_id = objects.alloc_id();
    objects.has_name.insert(obj_id, HasName($name.into()));
    objects
      .has_description
      .insert(obj_id, HasDescription($description.into()));
    objects.is_in_room.insert(obj_id, IsInRoom(None));
    obj_id
  }};
  ($name: expr, $description: expr, $in_room: expr) => {{
    use crate::component::{HasDescription, HasName, IsInRoom};
    use crate::object::OBJECTS;
    let mut objects = OBJECTS.lock().unwrap();
    let obj_id = objects.alloc_id();
    objects.has_name.insert(obj_id, HasName($name.into()));
    objects
      .has_description
      .insert(obj_id, HasDescription($description.into()));
    objects.is_in_room.insert(obj_id, IsInRoom(Some($in_room)));
    obj_id
  }};
}
