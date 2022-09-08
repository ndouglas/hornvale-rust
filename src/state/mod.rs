use specs::prelude::*;

pub mod run_mode;
pub use run_mode::*;

use crate::ecs::components::*;
use crate::ecs::dispatcher::{get_new_dispatcher, UnifiedDispatcher};
use crate::ecs::resources::*;

pub struct State {
  pub ecs: World,
  pub dispatcher: Box<dyn UnifiedDispatcher + 'static>,
  pub run_mode: RunMode,
}

impl State {

  #[named]
  pub fn new() -> Self {
    trace_enter!();
    let mut ecs = World::new();
    register_components(&mut ecs);
    insert_resources(&mut ecs);
    let result = State {
      ecs,
      dispatcher: get_new_dispatcher(),
      run_mode: RunMode::Initial,
    };
    trace_exit!();
    result
  }

  #[named]
  pub fn run_systems(&mut self) {
    trace_enter!();
    if self.run_mode.should_maintain_ecs() {
      self.dispatcher.run_now(&mut self.ecs);
      self.ecs.maintain();
      // collect_garbage(&mut self.ecs);
      self.ecs.maintain();
      // run_effects_queue(&mut self.ecs);
      self.ecs.maintain();
      let mut tick = self.ecs.write_resource::<Tick>();
      tick.0 += 1;
    }
    trace_exit!();
  }

  #[named]
  pub fn tick(&mut self) {
    trace_enter!();
    let ecs = &mut self.ecs;
    if let Some(new_mode) = self.run_mode.tick(ecs) {
      self.run_mode = new_mode;
    }
    self.run_systems();
    trace_exit!();
  }
}
