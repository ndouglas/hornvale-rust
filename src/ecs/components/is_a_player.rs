use serde::*;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Copy, Clone, Debug, Default, PartialEq)]
#[storage(NullStorage)]
pub struct IsAPlayer;
