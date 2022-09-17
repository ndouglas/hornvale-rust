use specs::prelude::*;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq)]
pub struct PlayerResource(pub Option<Entity>);
