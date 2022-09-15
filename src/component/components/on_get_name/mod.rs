use crate::entity::Entity;

pub type GetNameFunctionType = fn(Entity) -> Option<String>;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct OnGetName(pub GetNameFunctionType);
