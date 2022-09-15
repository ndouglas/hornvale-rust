use std::collections::HashSet;
use std::sync::Mutex;

use crate::entity::Entity;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
pub struct Object(Entity);

lazy_static! {
  pub static ref OBJECTS: Mutex<HashSet<Object>> = Mutex::new(HashSet::new());
}
