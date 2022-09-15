use std::collections::HashSet;

use crate::entity::Entity;

#[derive(Clone, Debug, PartialEq)]
pub struct HasVisitedRooms(pub HashSet<Entity>);
