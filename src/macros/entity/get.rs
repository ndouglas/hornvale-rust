#[macro_export]
macro_rules! get_player {
  ($ecs: expr) => {{
    use crate::ecs::resources::Player;
    $ecs.fetch::<Player>().0
  }};
}

#[macro_export]
macro_rules! get_spawn_room {
  ($ecs: expr) => {{
    use crate::ecs::resources::SpawnRoom;
    $ecs.fetch::<SpawnRoom>().0
  }};
}

#[macro_export]
macro_rules! get_tick {
  ($ecs: expr) => {{
    use crate::ecs::resources::Tick;
    *$ecs.fetch::<Tick>()
  }};
}

#[macro_export]
macro_rules! get_current_room {
  ($ecs: expr, $entity: expr) => {{
    use crate::ecs::components::IsInRoom;
    let mut result = None;
    if let Some(IsInRoom(room_entity)) = $ecs.read_storage::<IsInRoom>().get($entity) {
      result = Some(room_entity.to_owned());
    }
    result
  }};
}

#[macro_export]
macro_rules! get_name {
  ($ecs: expr, $entity: expr) => {{
    use crate::ecs::components::HasName;
    let mut result = None;
    if let Some(HasName(name)) = $ecs.read_storage::<HasName>().get($entity) {
      result = Some(name.to_owned());
    }
    result
  }};
}

#[macro_export]
macro_rules! get_description {
  ($ecs: expr, $entity: expr) => {{
    use crate::ecs::components::HasDescription;
    let mut result = None;
    if let Some(HasDescription(description)) = $ecs.read_storage::<HasDescription>().get($entity) {
      result = Some(description.to_owned());
    }
    result
  }};
}

#[macro_export]
macro_rules! get_exits {
  ($ecs: expr, $entity: expr) => {{
    use crate::ecs::components::HasExits;
    let mut result = None;
    if let Some(HasExits { exits }) = $ecs.read_storage::<HasExits>().get($entity) {
      result = Some(exits.to_owned());
    }
    result
  }};
}

#[macro_export]
macro_rules! get_exit_to {
  ($ecs: expr, $entity: expr, $direction: expr) => {{
    let mut result = None;
    if let Some(exits) = get_exits!($ecs, $entity) {
      if let Some(exit) = exits.get_exit($direction) {
        result = Some(exit.to_owned());
      }
    }
    result
  }};
}
