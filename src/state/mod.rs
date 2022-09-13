use colored::*;
use rustyline::{Editor, ExternalPrinter, Result};
use specs::prelude::*;
use std::thread;
use std::time::Duration;

pub mod run_mode;
pub use run_mode::*;

use crate::ecs::components::*;
use crate::ecs::dispatcher::{get_new_dispatcher, UnifiedDispatcher};
use crate::ecs::resources::*;
use crate::parser::parse;
use crate::queue::*;

pub struct State {
  pub ecs: World,
  pub dispatcher: Box<dyn UnifiedDispatcher + 'static>,
  pub run_mode: RunMode,
  pub editor: Editor<()>,
}

impl State {
  #[named]
  pub fn new(editor: Editor<()>) -> Self {
    let mut ecs = World::new();
    register_components(&mut ecs);
    insert_resources(&mut ecs);
    State {
      ecs,
      editor,
      dispatcher: get_new_dispatcher(),
      run_mode: RunMode::Initial,
    }
  }

  #[named]
  pub fn should_continue(&self) -> bool {
    self.run_mode.should_continue()
  }

  #[named]
  pub fn quit(&mut self) {
    self.run_mode = RunMode::Quit;
  }

  #[named]
  pub fn run_systems(&mut self) {
    if self.run_mode.should_maintain_ecs() {
      self.dispatcher.run_now(&mut self.ecs);
      self.ecs.maintain();
      run_command_queue(&mut self.ecs);
      self.ecs.maintain();
      run_action_queue(&mut self.ecs);
      self.ecs.maintain();
      run_effect_queue(&mut self.ecs);
      self.ecs.maintain();
      run_event_queue(&mut self.ecs);
      let mut tick = self.ecs.write_resource::<Tick>();
      tick.0 += 1;
    }
  }

  #[named]
  pub fn tick(&mut self) {
    let ecs = &mut self.ecs;
    if let Some(new_mode) = self.run_mode.tick(ecs) {
      self.run_mode = new_mode;
    }
    self.run_systems();
  }

  #[named]
  pub fn read_input(&mut self) {
    match self.editor.readline(format!("{} ", ">".blue()).as_str()) {
      Ok(line) => parse(line, self),
      Err(_) => {}
    }
  }
}
