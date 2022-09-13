use specs::prelude::*;

use crate::traits::Effectable;

pub mod move_entity;
pub use move_entity::*;
pub mod print_error;
pub use print_error::*;
pub mod print_room;
pub use print_room::*;

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Effect {
  MoveEntity(MoveEntityEffect),
  PrintError(PrintErrorEffect),
  PrintRoom(PrintRoomEffect),
}

impl Effectable for Effect {
  #[named]
  fn execute(&self, ecs: &mut World) {
    use Effect::*;
    match self {
      MoveEntity(effect) => effect.execute(ecs),
      PrintError(effect) => effect.execute(ecs),
      PrintRoom(effect) => effect.execute(ecs),
    }
  }
}
