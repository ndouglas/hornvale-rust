use crate::entity::Entity;

pub type GetDescriptionFunctionType = fn(Entity) -> Option<String>;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct OnGetDescription(pub GetDescriptionFunctionType);
