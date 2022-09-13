#[macro_export]
macro_rules! cmd_echo {
  ($entity: expr, $str: expr) => {
    Command::Echo(EchoCommand {
      entity: $entity,
      string: $str,
    })
  };
}

#[macro_export]
macro_rules! cmd_look {
  ($entity: expr) => {
    Command::Look(LookCommand { entity: $entity })
  };
}

#[macro_export]
macro_rules! cmd_move_to {
  ($entity: expr, $dir: expr) => {
    Command::MoveDirection(MoveDirectionCommand {
      entity: $entity,
      direction: $dir,
    })
  };
}

#[macro_export]
macro_rules! cmd_quit {
  ($entity: expr) => {
    Command::Quit(QuitCommand { entity: $entity })
  };
}
