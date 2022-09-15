use crate::room::Room;

pub type HasDescriptionFunctionType = fn(Room) -> String;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct HasDescription(pub HasDescriptionFunctionType);
