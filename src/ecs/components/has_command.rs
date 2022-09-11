use serde::*;
use specs::prelude::*;
use specs_derive::Component;

use crate::commands::Command;
use crate::traits::Commandable;

#[derive(Component, Debug, Hash, PartialEq)]
pub struct HasCommand(pub Command);

pub trait HasCommandBuilder {

  fn has_command(self, command: Command) -> Self;

}

impl HasCommandBuilder for EntityBuilder<'_> {

  fn has_command(self, command: Command) -> Self {
    self.with(HasCommand(command))
  }

}