use specs::prelude::*;

use crate::navigation::Direction;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum Action {
  Look { entity: Entity },
  MoveDirection { entity: Entity, direction: Direction },
}

impl Action {}
