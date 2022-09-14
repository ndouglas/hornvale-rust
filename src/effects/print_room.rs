use specs::prelude::*;

use super::Effectable;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct PrintRoomEffect {
  pub room: Entity,
}

impl Effectable for PrintRoomEffect {
  #[named]
  fn execute(&self) {
    print_room!(self.room);
  }
}
