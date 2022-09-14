use std::sync::Mutex;

pub mod storage;
pub use storage::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
pub struct Object(u64);

lazy_static! {
  pub static ref OBJECTS: Mutex<ObjectStorage> = Mutex::new(ObjectStorage::new());
}
