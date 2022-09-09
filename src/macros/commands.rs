#[macro_export]
macro_rules! cmd_move_to {
  ($var: ident) => {
    Command::MoveCompassDirection(MoveCompassDirection { compass_direction: CompassDirection::$var })
  }
}

#[macro_export]
macro_rules! cmd_look {
  () => {
    Command::Look(Look { })
  }
}
