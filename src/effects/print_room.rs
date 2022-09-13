use specs::prelude::*;

use crate::traits::Effectable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct PrintRoomEffect {
  pub room: Entity,
}

impl Effectable for PrintRoomEffect {
  #[named]
  fn execute(&self, _ecs: &mut World) {
    print_room!(self.room);
  }
}
