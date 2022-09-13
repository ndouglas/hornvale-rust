#[macro_export]
macro_rules! create_room {
  ($ecs: expr, $name: expr, $description: expr) => {
    $ecs
      .create_entity()
      .has_name(format!("{}", $name))
      .has_description(format!("{}", $description))
      .is_a_room()
      .has_exits()
      .build()
  };
}

#[macro_export]
macro_rules! create_player {
  ($ecs: expr, $name: expr, $description: expr, $in_room: expr) => {{
    let player = $ecs
      .create_entity()
      .has_name(format!("{}", $name))
      .has_description(format!("{}", $description))
      .is_a_player()
      .is_in_room($in_room)
      .build();
    $ecs.insert(Player(player));
    player
  }};
}
