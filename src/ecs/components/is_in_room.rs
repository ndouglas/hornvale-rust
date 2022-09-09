use serde::*;
use specs::prelude::*;
use specs::world::Index;
use specs_derive::Component;

#[derive(Component, Copy, Clone, Debug, Hash, PartialEq)]
pub struct IsInRoom(pub Entity);
