#[macro_export]
macro_rules! create_player {
  ($name: expr, $description: expr, $in_room: expr) => {{
    use std::collections::HashSet;
    use crate::component::*;
    use crate::entity::ENTITIES;
    use crate::player::PLAYER;
    let player_id = {
      let mut entities = ENTITIES.lock().unwrap();
      let player_id = entities.alloc_id();
      entities.has_name.insert(player_id, HasName($name.into()));
      entities
        .has_description
        .insert(player_id, HasDescription({ |_room_id| $description.into() }));
      entities.is_in_room.insert(player_id, IsInRoom(Some($in_room)));
      entities.has_visited_rooms.insert(player_id, HasVisitedRooms(HashSet::new()));
      player_id
    };
    PLAYER.lock().unwrap().0 = Some(player_id);
    player_id
  }};
}
