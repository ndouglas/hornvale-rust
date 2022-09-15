use crate::entity::Entity;

#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub struct IsInRoom(pub Option<Entity>);
