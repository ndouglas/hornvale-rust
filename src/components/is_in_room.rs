use specs::prelude::*;

use specs_derive::Component;

#[derive(Component, Copy, Clone, Debug, Hash, PartialEq)]
pub struct IsInRoom(pub Entity);

pub trait IsInRoomBuilder {
  fn put_in_room(self, room: Entity) -> Self;
}

impl IsInRoomBuilder for EntityBuilder<'_> {
  #[named]
  fn put_in_room(self, room: Entity) -> Self {
    self.with(IsInRoom(room))
  }
}
