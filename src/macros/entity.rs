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
