#[macro_export]
macro_rules! create_exit {
  (@inner $system_data: expr, $from: expr, $to: expr, $direction: expr) => {{
    use crate::navigation::Exit;
    if let Some(has_exits) = $system_data.has_exits.get_mut($from) {
      has_exits.exits.set_exit(
        $direction,
        Some(Exit {
          direction: $direction.to_owned(),
          from: $from,
          to: $to,
          is_passable: true,
        }),
      );
    }
  }};
  ($system_data: expr, $from: expr, $to: expr, $direction: expr, $bidirectional: expr) => {{
    create_exit!(@inner $system_data, $from, $to, $direction);
    if $bidirectional {
      create_exit!(@inner $system_data, $to, $from, &$direction.get_inverse());
    }
  }};
}

#[macro_export]
macro_rules! get_exits {
  ($system_data: expr, $room: expr) => {{
    let mut result = None;
    if let Some(HasExits { exits }) = $system_data.has_exits.get($room) {
      result = Some(exits.to_owned());
    }
    result
  }};
}
