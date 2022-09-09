use specs::prelude::*;

use crate::actions::*;
use crate::commands::Command;
use crate::ecs::components::*;
use crate::model::CompassDirection;
use crate::traits::Actionable;
use crate::traits::Commandable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct MoveCompassDirectionAction {
  pub entity: Entity,
  pub compass_direction: CompassDirection,
}

impl Actionable for MoveCompassDirectionAction {}
