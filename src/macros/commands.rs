#[macro_export]
macro_rules! cmd_move_to {
  ($entity: ident, $dir: ident) => {
    Command::MoveCompassDirection(MoveCompassDirectionCommand {
      entity: $entity,
      compass_direction: CompassDirection::$dir,
    })
  };
}

#[macro_export]
macro_rules! cmd_look {
  ($entity: ident) => {
    Command::Look(LookCommand { entity: $entity })
  };
}

#[macro_export]
macro_rules! cmd_echo {
  ($entity: ident, $str: expr) => {
    Command::Echo(EchoCommand { entity: $entity, string: $str })
  };
}
