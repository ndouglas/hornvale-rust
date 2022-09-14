use std::sync::Mutex;

pub mod storage;
pub use storage::*;

use crate::room::ROOMS;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
pub struct Entity(u64);

lazy_static! {
  pub static ref ENTITIES: Mutex<EntityStorage> = Mutex::new(EntityStorage::new());
}
