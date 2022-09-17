use specs::prelude::*;
use specs_derive::*;

#[derive(Clone, Copy, Component, Debug, Default, Hash, PartialEq)]
#[storage(NullStorage)]
pub struct IsARoom;
