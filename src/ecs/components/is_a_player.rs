use serde::*;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Copy, Clone, Debug, Default, PartialEq)]
#[storage(NullStorage)]
pub struct IsAPlayer;

pub trait IsAPlayerBuilder {
  fn is_a_player(self) -> Self;
}

impl IsAPlayerBuilder for EntityBuilder<'_> {
  #[named]
  fn is_a_player(self) -> Self {
    self.with(IsAPlayer)
  }
}
