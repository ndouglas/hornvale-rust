#[macro_export]
macro_rules! create_room {
  ($name: expr, $description: expr) => {{
    use crate::component::{ HasDescription, HasName, HasExits };
    use crate::room::ROOMS;
    let mut rooms = ROOMS.lock().unwrap();
    let room_id = rooms.alloc_id();
    rooms.has_name.insert(room_id, HasName($name.into()));
    rooms.has_description.insert(room_id, HasDescription($description.into()));
    rooms.has_exits.insert(room_id, HasExits::default());
    room_id
  }};
}

#[macro_export]
macro_rules! create_exit_inner {
  ($from: expr, $to: expr, $direction: expr) => {{
    use crate::component::{ HasDescription, HasName, HasExits };
    use crate::model::{ Direction, Exit };
    use crate::room::ROOMS;
    let mut rooms = ROOMS.lock().unwrap();
    if let Some(has_exits) = rooms.has_exits.get_opt_mut($from) {
      has_exits.exits.set_exit(
        $direction,
        Some(Exit {
          direction: $direction.to_owned(),
          room_entity: $to,
          is_passable: true,
        }),
      );
    }
  }};
}


#[macro_export]
macro_rules! create_exit {
  ($from: expr, $to: expr, $direction: expr, $bidirectional: expr) => {{
    create_exit_inner!($from, $to, $direction);
    if $bidirectional {
      create_exit_inner!($to, $from, &$direction.get_inverse());
    }
  }};
}

#[macro_export]
macro_rules! create_player {
  ($name: expr, $description: expr, $in_room: expr) => {{
    use crate::component::{ HasDescription, HasName, IsInRoom };
    use crate::entity::{ Entity, ENTITIES };
    use crate::player::PLAYER;
    let player_id = {
      let mut entities = ENTITIES.lock().unwrap();
      let player_id = entities.alloc_id();
      entities.has_name.insert(player_id, HasName($name.into()));
      entities.has_description.insert(player_id, HasDescription($description.into()));
      entities.is_in_room.insert(player_id, IsInRoom($in_room));
      player_id
    };
    PLAYER.lock().unwrap().0 = Some(player_id);
    player_id
  }};
}
