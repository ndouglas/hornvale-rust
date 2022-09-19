use specs::prelude::*;
use std::str::FromStr;

use crate::action::Action;
use crate::navigation::Direction;

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Command {
  Echo {
    entity: Entity,
    string: String,
    original_input: String,
  },
  Look {
    entity: Entity,
    original_input: String,
  },
  LookDirection {
    entity: Entity,
    direction: Direction,
    original_input: String,
  },
  LookAtObject {
    entity: Entity,
    object: Entity,
    original_input: String,
  },
  MoveDirection {
    entity: Entity,
    direction: Direction,
    original_input: String,
  },
  Quit {
    entity: Entity,
    original_input: String,
  },
}

impl Command {
  #[named]
  pub fn get_action(&self) -> Result<Action, ()> {
    use Command::*;
    match self {
      Look { entity, .. } => Ok(Action::Look { entity: *entity }),
      LookDirection { entity, direction, .. } => Ok(Action::LookDirection {
        entity: *entity,
        direction: *direction,
      }),
      LookAtObject { entity, object, .. } => Ok(Action::LookAtObject {
        entity: *entity,
        object: *object,
      }),
      MoveDirection { entity, direction, .. } => Ok(Action::MoveDirection {
        entity: *entity,
        direction: *direction,
      }),
      _ => Err(()),
    }
  }
}
