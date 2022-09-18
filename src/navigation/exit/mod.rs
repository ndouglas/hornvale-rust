use specs::prelude::*;
use std::fmt;

use crate::navigation::Direction;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Exit {
  pub direction: Direction,
  pub from: Entity,
  pub to: Entity,
  pub is_passable: bool,
}

impl fmt::Display for Exit {
  #[named]
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "{}", self.direction.get_lowercase())
  }
}
