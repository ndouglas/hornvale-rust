use specs::prelude::*;

use crate::commands::Command;
use crate::ecs::components::*;
use crate::model::CompassDirection;
use crate::traits::Commandable;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct MoveCompassDirection {
  pub compass_direction: CompassDirection,
}

impl Commandable for MoveCompassDirection {}

pub fn move_compass_direction(ecs: &mut World, entity: Entity, dir: CompassDirection) {
  let command = Command::MoveCompassDirection(MoveCompassDirection {
    compass_direction: dir,
  });
  ecs
    .write_storage::<HasCommand>()
    .insert(entity, HasCommand { command })
    .expect(format!("Could not move {:?} {}", entity, dir).as_str());
}
