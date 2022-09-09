use specs::prelude::World;
use std::fmt;

use crate::ecs::systems::*;

#[cfg(target_arch = "wasm32")]
#[macro_use]
mod single_thread;

#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
mod multi_thread;

#[cfg(target_arch = "wasm32")]
pub use single_thread::*;

#[cfg(not(target_arch = "wasm32"))]
pub use multi_thread::*;

pub trait UnifiedDispatcher {
  fn run_now(&mut self, ecs: *mut World);
}

impl fmt::Display for Box<dyn UnifiedDispatcher + 'static> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "UnifiedDispatcher")
  }
}

construct_dispatcher!(
  (Command, "command", &[]),
  (Action, "action", &[]),
  (Movement, "movement", &[])
);

#[named]
pub fn get_new_dispatcher() -> Box<dyn UnifiedDispatcher + 'static> {
  trace_enter!();
  let result = new_dispatch();
  trace_exit!();
  result
}
