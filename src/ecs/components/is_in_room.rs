use serde::*;
use specs::prelude::*;
use specs::world::Index;
use specs_derive::Component;

#[derive(Component, Copy, Clone, Debug, Hash, PartialEq)]
pub struct IsInRoom(pub Entity);

pub trait IsInRoomBuilder {
  fn is_in_room(self, room: Entity) -> Self;
}

impl IsInRoomBuilder for EntityBuilder<'_> {
  fn is_in_room(self, room: Entity) -> Self {
    self.with(IsInRoom(room))
  }
}
