use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Copy, Clone, Debug, Default, PartialEq)]
#[storage(NullStorage)]
pub struct IsAPlayer;

pub trait IsAPlayerBuilder {
  fn make_a_player(self) -> Self;
}

impl IsAPlayerBuilder for EntityBuilder<'_> {
  #[named]
  fn make_a_player(self) -> Self {
    self.with(IsAPlayer)
  }
}
