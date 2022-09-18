use specs::prelude::*;
use std::collections::HashSet;
use std::collections::VecDeque;

use crate::resource::resources::*;

#[named]
pub fn insert_resources(ecs: &mut World) {
  ecs.insert(MessagesResource(VecDeque::new()));
  ecs.insert(PlayerResource(None));
  ecs.insert(TickResource(0));
  ecs.insert(VisitedRoomsResource(HashSet::new()));
}
