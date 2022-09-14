#[macro_export]
macro_rules! act_look {
  ($entity: expr) => {{
    use crate::actions::Action;
    use crate::actions::look::LookAction;
    Action::Look(LookAction { entity: $entity })
  }};
}

#[macro_export]
macro_rules! act_move_direction {
  ($entity: expr, $dir: expr) => {{
    use crate::actions::Action;
    use crate::actions::move_direction::MoveDirectionAction;
    Action::MoveDirection(MoveDirectionAction {
      entity: $entity,
      direction: $dir,
    })
  }};
}
