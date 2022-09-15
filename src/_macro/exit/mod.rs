#[macro_export]
macro_rules! create_exit_inner {
  ($from: expr, $to: expr, $direction: expr) => {{
    use crate::model::{Direction, Exit};
    use crate::entity::ENTITIES;
    let mut entities = ENTITIES.lock().unwrap();
    if let Some(has_exits) = entities.has_exits.get_opt_mut($from) {
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
macro_rules! get_exits {
  ($room: expr) => {{
    use crate::component::HasExits;
    use crate::entity::ENTITIES;
    let mut result = None;
    let entities = ENTITIES.lock().unwrap();
    if let Some(HasExits { exits }) = entities.has_exits.get_opt($room) {
      result = Some(exits.to_owned());
    }
    result
  }};
}

#[macro_export]
macro_rules! get_exit_to {
  ($room: expr, $direction: expr) => {{
    let mut result = None;
    if let Some(exits) = get_exits!($room) {
      if let Some(exit) = exits.get_exit($direction) {
        result = Some(exit.to_owned());
      }
    }
    result
  }};
}
