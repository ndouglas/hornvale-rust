use specs::prelude::*;
use std::fmt;

use crate::model::CompassDirection;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct RoomExit {
  pub compass_direction: CompassDirection,
  pub room_entity: Entity,
}
