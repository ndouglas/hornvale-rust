#[macro_export]
macro_rules! act_look {
  ($entity: expr) => {{
    Action::Look(LookAction { entity: $entity })
  }};
}

#[macro_export]
macro_rules! act_move_direction {
  ($entity: expr, $dir: expr) => {{
    Action::MoveDirection(MoveDirectionAction {
      entity: $entity,
      direction: $dir,
    })
  }};
}
