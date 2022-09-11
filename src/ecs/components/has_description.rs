use serde::*;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Clone, Debug, Hash, PartialEq)]
pub struct HasDescription(pub String);

pub trait HasDescriptionBuilder {
  fn has_description(self, description: String) -> Self;
}

impl HasDescriptionBuilder for EntityBuilder<'_> {
  fn has_description(self, description: String) -> Self {
    self.with(HasDescription(description))
  }
}
