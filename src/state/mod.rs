use colored::*;
use rustyline::Editor;
use specs::prelude::*;

use crate::commands::Command;
use crate::ecs::components::*;
use crate::ecs::resources::*;
use crate::queue::*;

pub struct State {
  pub ecs: World,
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
    }
  }

  #[named]
  pub fn should_continue(&self) -> bool {
    self.ecs.read_resource::<RunMode>().should_continue()
  }

  #[named]
  pub fn run_systems(&mut self) {
    run_command_queue(&mut self.ecs);
    run_action_queue(&mut self.ecs);
    run_effect_queue(&mut self.ecs);
    run_event_queue(&mut self.ecs);
    let mut tick = self.ecs.write_resource::<Tick>();
    tick.0 += 1;
  }

  #[named]
  pub fn tick(&mut self) {
    self.run_systems();
  }

  #[named]
  pub fn read_input(&mut self) {
    match self.editor.readline(format!("{} ", ">".blue()).as_str()) {
      Ok(line) => {
        let player_entity = get_player!(self.ecs);
        match Command::from_str(&line, player_entity) {
          Ok(command) => enq_command!(command),
          Err(_) => enq_message!(format!("{}", "What?".bright_red())),
        }
      },
      Err(_) => {},
    }
  }
}
