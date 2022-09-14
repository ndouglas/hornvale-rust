#[macro_export]
macro_rules! create_room {
  ($ecs: expr, $name: expr, $description: expr) => {{
    $ecs
      .create_entity()
      .give_name(format!("{}", $name))
      .give_description(format!("{}", $description))
      .make_a_room()
      .give_exits()
      .build()
  }};
}

#[macro_export]
macro_rules! create_player {
  ($ecs: expr, $name: expr, $description: expr, $in_room: expr) => {{
    use crate::player::Player;
    let player = $ecs
      .create_entity()
      .give_name(format!("{}", $name))
      .give_description(format!("{}", $description))
      .make_a_player()
      .put_in_room($in_room)
      .build();
    $ecs.insert(Player(Some(player)));
    player
  }};
}
