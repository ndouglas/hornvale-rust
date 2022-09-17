use specs::prelude::*;
use specs_derive::*;

#[derive(Copy, Clone, Component, Debug, Default, Hash, PartialEq)]
#[storage(NullStorage)]
pub struct IsAnObject;
