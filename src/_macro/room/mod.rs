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
macro_rules! create_room {
  ($name: expr, $description: expr) => {{
    use crate::component::{HasBeenVisited, HasDescription, HasExits, HasName};
    use crate::room::ROOMS;
    let mut rooms = ROOMS.lock().unwrap();
    let room_id = rooms.alloc_id();
    rooms.has_name.insert(room_id, HasName($name.into()));
    rooms.has_description.insert(
      room_id,
      HasDescription(|room_id| {
        let mut rooms = ROOMS.lock().unwrap();
        if let Some(has_been_visited) = rooms.has_been_visited.get_opt(room_id) {
          format!("{}", "You've already seen this!")
        } else {
          rooms.has_been_visited.insert(room_id, HasBeenVisited);
          $description.into()
        }
      }),
    );
    rooms.has_exits.insert(room_id, HasExits::default());
    room_id
  }};
}

#[macro_export]
macro_rules! format_room {
  ($room: expr) => {{
    use crate::component::IsInRoom;
    use crate::entity::ENTITIES;
    use crate::object::OBJECTS;
    use colored::*;
    let mut string = String::new();
    if let Some(name) = get_room_name!($room) {
      string.push_str(format!("{}\n", name.magenta()).as_str());
    }
    if let Some(description) = get_room_description!($room) {
      string.push_str(format!("{}\n", description($room).green()).as_str());
    }
    {
      let objects = OBJECTS.lock().unwrap();
      for id in objects.is_in_room.ids_collected() {
        if IsInRoom(Some($room)) == *objects.is_in_room.get(id) {
          if let Some(description) = objects.has_description.get_opt(id) {
            string.push_str(format!("{}\n", description.0($room).blue()).as_str());
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
    enq_event!(evt_print_room!($room));
  }};
}
