use crate::room::Room;

#[derive(Copy, Clone, Debug, Hash, PartialEq)]
pub struct IsInRoom(pub Option<Room>);
