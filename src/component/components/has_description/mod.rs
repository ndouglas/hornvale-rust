use crate::entity::Entity;

pub type HasDescriptionFunctionType = fn(Entity) -> String;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct HasDescription(pub HasDescriptionFunctionType);
