use specs::prelude::*;

use crate::commands::Command;
use crate::ecs::components::*;
use crate::model::CompassDirection;
use crate::traits::Commandable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct MoveCompassDirection {
  pub compass_direction: CompassDirection,
}

impl Commandable for MoveCompassDirection {}
