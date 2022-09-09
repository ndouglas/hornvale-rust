use serde::*;
use specs::prelude::*;
use specs_derive::Component;

use crate::model::CompassDirection;

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct WantsToMove {
  pub compass_direction: CompassDirection,
}
