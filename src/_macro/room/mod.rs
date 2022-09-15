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
    entities.is_a_room.insert(room_id, IsARoom);
    room_id
  }};
}

#[macro_export]
macro_rules! format_room {
  ($room: expr) => {{
    use crate::component::*;
    use crate::entity::ENTITIES;
    use colored::*;
    use std::collections::HashSet;
    let mut string = String::new();
    if let Some(name) = get_name!($room) {
      string.push_str(format!("{}\n", name.magenta()).as_str());
    }
    if let Some(description) = get_description!($room) {
      string.push_str(format!("{}\n", description.green()).as_str());
    }
    {
      let ids: HashSet<Entity> = {
        let entities = ENTITIES.lock().unwrap();
        let entities_in_a_room = entities
          .is_in_room
          .ids_collected()
          .into_iter()
          .collect::<HashSet<Entity>>();
        let object_entities = entities
          .is_an_object
          .ids_collected()
          .into_iter()
          .collect::<HashSet<Entity>>();
        entities_in_a_room
          .intersection(&object_entities)
          .into_iter()
          .map(|entity| *entity)
          .collect()
      };
      for id in ids {
        if let Some(Some(room)) = get_current_room!(id) {
          if room == $room {
            if let Some(description) = get_description!(id) {
              string.push_str(format!("{}\n", description.blue()).as_str());
            }
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
