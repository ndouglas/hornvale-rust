use specs::prelude::*;
use std::fmt;

use crate::model::Direction;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Exit {
  pub direction: Direction,
  pub room_entity: Entity,
  pub is_passable: bool,
}

impl fmt::Display for Exit {
  #[named]
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    trace_enter!();
    let result = write!(formatter, "{}", self.direction.get_lowercase());
    trace_exit!();
    result
  }
}
