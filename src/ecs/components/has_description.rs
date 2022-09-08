use serde::*;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Clone, Debug, Hash, PartialEq)]
pub struct HasDescription {
  pub description: String,
}
