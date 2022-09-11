use specs::prelude::*;
use std::fmt;

use crate::model::Direction;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Exit {
  pub direction: Direction,
  pub room_entity: Entity,
  pub is_passable: bool,
}
