#[macro_export]
macro_rules! cmd_echo {
  ($entity: expr, $string: expr) => {{
    Command::Echo(EchoCommand {
      entity: $entity,
      string: $string,
    })
  }};
}

#[macro_export]
macro_rules! cmd_look {
  ($entity: expr) => {{
    Command::Look(LookCommand { entity: $entity })
  }};
}

#[macro_export]
macro_rules! cmd_move_to {
  ($entity: expr, $direction: expr) => {{
    Command::MoveDirection(MoveDirectionCommand {
      entity: $entity,
      direction: $direction,
    })
  }};
}

#[macro_export]
macro_rules! cmd_quit {
  ($entity: expr) => {{
    Command::Quit(QuitCommand { entity: $entity })
  }};
}

#[macro_export]
macro_rules! enq_command {
  ($command: expr) => {{
    crate::command::enqueue_command($command)
  }};
}
