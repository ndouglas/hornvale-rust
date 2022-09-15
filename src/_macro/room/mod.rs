#[macro_export]
macro_rules! create_room {
  ($name: expr, $description: expr) => {{
    use crate::component::*;
    use crate::entity::ENTITIES;
    let mut entities = ENTITIES.lock().unwrap();
    let room_id = entities.alloc_id();
    entities.has_name.insert(room_id, HasName($name.into()));
    entities
      .has_description
      .insert(room_id, HasDescription($description.into()));
    entities.has_exits.insert(room_id, HasExits::default());
    room_id
  }};
}

#[macro_export]
macro_rules! format_room {
  ($room: expr) => {{
    use crate::component::*;
    use crate::entity::ENTITIES;
    use colored::*;
    let mut string = String::new();
    if let Some(name) = get_name!($room) {
      string.push_str(format!("{}\n", name.magenta()).as_str());
    }
    if let Some(description) = get_description!($room) {
      string.push_str(format!("{}\n", description.green()).as_str());
    }
    {
      let entities = ENTITIES.lock().unwrap();
      for id in entities.is_in_room.ids_collected() {
        if IsInRoom(Some($room)) == *entities.is_in_room.get(id) {
          if let Some(description) = entities.has_description.get_opt(id) {
            string.push_str(format!("{}\n", description.0.blue()).as_str());
          }
        }
      }
    }
    if let Some(exits) = get_exits!($room) {
      string.push_str(format!("{}\n", format!("{}", exits).bright_green()).as_str());
    }
    format!("{}", string)
  }};
}

#[macro_export]
macro_rules! print_room {
  ($room: expr) => {{
    enq_message!(format_room!($room));
  }};
}
