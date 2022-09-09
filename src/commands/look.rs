use specs::prelude::*;

use crate::commands::Command;
use crate::ecs::components::*;
use crate::model::CompassDirection;
use crate::traits::Commandable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct Look {
}

impl Commandable for Look {}
