use specs::prelude::*;

use crate::traits::Effectable;

pub mod look;
pub use look::*;
pub mod move_entity;
pub use move_entity::*;

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Effect {
  Look(LookEffect),
  MoveEntity(MoveEntityEffect),
}

impl Effectable for Effect {
  #[named]
  fn execute(&self, ecs: &mut World) {
    use Effect::*;
    match self {
      Look(effect) => effect.execute(ecs),
      MoveEntity(effect) => effect.execute(ecs),
    }
  }
}
