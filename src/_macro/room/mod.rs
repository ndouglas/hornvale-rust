#[macro_export]
macro_rules! get_room_name {
  ($room: expr) => {{
    use crate::component::*;
    use crate::entity::ENTITIES;
    let mut result = None;
    let entities = ENTITIES.lock().unwrap();
    if let Some(HasName(name)) = entities.has_name.get_opt($room) {
      result = Some(name.to_owned());
    }
    result
  }};
}

#[macro_export]
macro_rules! get_room_description {
  ($room: expr) => {{
    use crate::component::*;
    use crate::entity::ENTITIES;
    let mut result = None;
    let entities = ENTITIES.lock().unwrap();
    if let Some(HasDescription(description)) = entities.has_description.get_opt($room) {
      result = Some(description.to_owned());
    }
    result
  }};
}

#[macro_export]
macro_rules! create_room {
  ($name: expr, $description: expr) => {{
    use crate::component::*;
    use crate::entity::ENTITIES;
    let mut entities = ENTITIES.lock().unwrap();
    let room_id = entities.alloc_id();
    entities.has_name.insert(room_id, HasName($name.into()));
    entities.has_description.insert(
      room_id,
      HasDescription(|room_id| {
        use crate::entity::ENTITIES;
        use crate::player::PLAYER;
        if let Some(player_id) = PLAYER.lock().unwrap().0 {
          let mut entities = ENTITIES.lock().unwrap();
          if let Some(has_visited_rooms) = entities.has_visited_rooms.get_opt_mut(player_id) {
            if has_visited_rooms.0.contains(&room_id) {
              format!("{}", "You've already seen this!")
            } else {
              has_visited_rooms.0.insert(room_id.clone());
              $description.into()
            }
          } else {
            $description.into()
          }  
        } else {
          $description.into()
        }
      }),
    );
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
    if let Some(name) = get_room_name!($room) {
      string.push_str(format!("{}\n", name.magenta()).as_str());
    }
    if let Some(description) = get_room_description!($room) {
      string.push_str(format!("{}\n", description($room).green()).as_str());
    }
    {
      let entities = ENTITIES.lock().unwrap();
      for id in entities.is_in_room.ids_collected() {
        if IsInRoom(Some($room)) == *entities.is_in_room.get(id) {
          if let Some(description) = entities.has_description.get_opt(id) {
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
    enq_message!(format_room!($room));
  }};
}
