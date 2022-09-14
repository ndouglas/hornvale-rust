#[macro_export]
macro_rules! act_look {
  ($entity: expr) => {{
    use crate::action::look::LookAction;
    use crate::action::Action;
    Action::Look(LookAction { entity: $entity })
  }};
}

#[macro_export]
macro_rules! act_move_direction {
  ($entity: expr, $dir: expr) => {{
    use crate::action::move_direction::MoveDirectionAction;
    use crate::action::Action;
    Action::MoveDirection(MoveDirectionAction {
      entity: $entity,
      direction: $dir,
    })
  }};
}
