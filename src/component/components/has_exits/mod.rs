use specs::prelude::*;
use specs_derive::*;

use crate::navigation::Exits;

#[derive(Clone, Copy, Component, Debug, Default, PartialEq)]
pub struct HasExits {
  pub exits: Exits,
}
