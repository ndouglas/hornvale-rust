#[macro_export]
macro_rules! create_room {
  ($system_data: expr, $name: expr, $description: expr) => {{
    use crate::component::*;
    let room_id = $system_data.entities.create();
    $system_data
      .has_name
      .insert(room_id, HasName($name.into()))
      .expect(&format!("Unable to insert name {} for entity!", $name));
    $system_data
      .has_description
      .insert(room_id, HasDescription($description.into()))
      .expect(&format!("Unable to insert description {} for entity!", $description));
    $system_data
      .has_exits
      .insert(room_id, HasExits::default())
      .expect("Unable to insert exits for entity!");
    $system_data
      .is_a_room
      .insert(room_id, IsARoom)
      .expect("Unable to insert is-a-room for entity!");
    room_id
  }};
}

#[macro_export]
macro_rules! format_room {
  ($system_data: expr, $room: expr) => {{
    use colored::*;
    let mut string = String::new();
    if let Some(name) = get_name!($system_data, $room) {
      string.push_str(format!("{}\n", name.magenta()).as_str());
    }
    if let Some(description) = get_description!($system_data, $room) {
      string.push_str(format!("{}\n", description.yellow()).as_str());
    }
    {
      for (_entities, _is_in_room, _is_an_object, has_description) in (
        &$system_data.entities,
        &$system_data.is_in_room,
        &$system_data.is_an_object,
        &$system_data.has_description,
      )
        .join()
        .filter(|(_entity, is_in_room, _is_an_object, _has_description)| is_in_room.0 == Some($room))
      {
        string.push_str(format!("{}\n", has_description.0.bright_blue()).as_str());
      }
    }
    if let Some(exits) = get_exits!($system_data, $room) {
      string.push_str(format!("{}\n", format!("{}", exits).green()).as_str());
    }
    format!("{}", string)
  }};
}
