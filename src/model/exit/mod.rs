use std::fmt;

use crate::model::Direction;
use crate::entity::Entity;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Exit {
  pub direction: Direction,
  pub room_entity: Entity,
  pub is_passable: bool,
}

impl fmt::Display for Exit {
  #[named]
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "{}", self.direction.get_lowercase())
  }
}
