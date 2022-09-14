#[macro_export]
macro_rules! act_look {
  ($entity: expr) => {{
    use crate::actions::look::LookAction;
    use crate::actions::Action;
    Action::Look(LookAction { entity: $entity })
  }};
}

#[macro_export]
macro_rules! act_move_direction {
  ($entity: expr, $dir: expr) => {{
    use crate::actions::move_direction::MoveDirectionAction;
    use crate::actions::Action;
    Action::MoveDirection(MoveDirectionAction {
      entity: $entity,
      direction: $dir,
    })
  }};
}
