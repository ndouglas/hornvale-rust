use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Clone, Debug, Hash, PartialEq)]
pub struct HasName(pub String);

pub trait HasNameBuilder {
  fn give_name(self, name: String) -> Self;
}

impl HasNameBuilder for EntityBuilder<'_> {
  #[named]
  fn give_name(self, name: String) -> Self {
    self.with(HasName(name))
  }
}