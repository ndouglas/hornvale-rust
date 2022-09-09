#[macro_export]
macro_rules! cmd_move_to {
  ($var: ident) => {
    Command::MoveCompassDirection(MoveCompassDirectionCommand {
      compass_direction: CompassDirection::$var,
    })
  };
}

#[macro_export]
macro_rules! cmd_look {
  () => {
    Command::Look(LookCommand {})
  };
}
