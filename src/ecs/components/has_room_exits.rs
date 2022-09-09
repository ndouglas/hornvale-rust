use serde::*;
use specs::prelude::*;
use specs::prelude::*;
use specs_derive::Component;
use std::collections::HashMap;

use crate::model::CompassDirection;
use crate::model::RoomExit;

#[derive(Component, Clone, Debug, PartialEq)]
pub struct HasRoomExits {
  pub room_exits: HashMap<CompassDirection, RoomExit>,
}
