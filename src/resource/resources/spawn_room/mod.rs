use specs::prelude::*;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq)]
pub struct SpawnRoomResource(pub Option<Entity>);
