use serde::*;
use specs::prelude::*;
use specs::prelude::*;
use specs_derive::Component;
use std::collections::HashMap;

use crate::model::Direction;
use crate::model::RoomExit;

#[derive(Component, Clone, Debug, PartialEq)]
pub struct HasRoomExits(pub HashMap<Direction, RoomExit>);
