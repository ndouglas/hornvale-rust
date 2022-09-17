use specs::prelude::*;
use specs_derive::*;

#[derive(Clone, Copy, Component, Debug, Hash, PartialEq)]
pub struct IsInRoom(pub Option<Entity>);
