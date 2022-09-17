use specs::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct VisitedRoomsResource(pub HashSet<Entity>);
