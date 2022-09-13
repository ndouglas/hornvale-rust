use specs::prelude::*;
use specs_derive::Component;

use crate::actions::Action;

#[derive(Component, Debug, Hash, PartialEq)]
pub struct HasAction(pub Action);

pub trait HasActionBuilder {
  fn has_action(self, action: Action) -> Self;
}

impl HasActionBuilder for EntityBuilder<'_> {
  #[named]
  fn has_action(self, action: Action) -> Self {
    self.with(HasAction(action))
  }
}
