use serde::*;
use specs::prelude::*;
use specs::prelude::*;
use specs_derive::Component;

use crate::model::CompassDirection;
use crate::model::RoomExit;

#[derive(Component, Clone, Debug, Hash, PartialEq)]
pub struct HasRoomExits {
  pub room_exits: Vec<RoomExit>,
}
