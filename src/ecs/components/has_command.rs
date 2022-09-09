use serde::*;
use specs::prelude::*;
use specs_derive::Component;

use crate::commands::Command;
use crate::traits::Commandable;

#[derive(Component, Debug, Hash, PartialEq)]
pub struct HasCommand {
  pub command: Command,
}
