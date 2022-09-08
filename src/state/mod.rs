use specs::prelude::*;

pub mod run_mode;
pub use run_mode::*;

use crate::ecs::components::*;
use crate::ecs::dispatcher::{get_new_dispatcher, UnifiedDispatcher};
use crate::ecs::resources::Tick;

pub struct State {
  pub ecs: World,
  pub dispatcher: Box<dyn UnifiedDispatcher + 'static>,
  pub run_mode: RunMode,
}

impl State {
  pub fn new() -> Self {
    let mut ecs = World::new();
    register_components(&mut ecs);
    ecs.insert(Tick(0));
    State {
      ecs,
      dispatcher: get_new_dispatcher(),
      run_mode: RunMode::Initial,
    }
  }

  pub fn run_systems(&mut self) {
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
  }

  pub fn tick(&mut self) {
    let ecs = &mut self.ecs;
    if let Some(new_mode) = self.run_mode.tick(ecs) {
      self.run_mode = new_mode;
    }
    self.run_systems();
  }
}
