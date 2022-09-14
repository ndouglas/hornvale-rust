use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Copy, Clone, Debug, Default, PartialEq)]
#[storage(NullStorage)]
pub struct IsARoom;

pub trait IsARoomBuilder {
  fn make_a_room(self) -> Self;
}

impl IsARoomBuilder for EntityBuilder<'_> {
  #[named]
  fn make_a_room(self) -> Self {
    self.with(IsARoom)
  }
}
