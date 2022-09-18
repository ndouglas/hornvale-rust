use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

use crate::command::Command;

use crate::event::{ActionEvent, CommandEvent, OutputEvent};

pub struct ProcessCommandSystem {
  pub reader_id: ReaderId<CommandEvent>,
}

impl ProcessCommandSystem {}

#[derive(SystemData)]
pub struct ProcessCommandSystemData<'a> {
  pub entities: Entities<'a>,
  pub action_event_channel: Write<'a, EventChannel<ActionEvent>>,
  pub command_event_channel: Read<'a, EventChannel<CommandEvent>>,
  pub output_event_channel: Write<'a, EventChannel<OutputEvent>>,
}

impl<'a> System<'a> for ProcessCommandSystem {
  type SystemData = ProcessCommandSystemData<'a>;

  fn run(&mut self, mut data: Self::SystemData) {
    for event in data.command_event_channel.read(&mut self.reader_id) {
      let CommandEvent { command } = event;
      use Command::*;
      match command {
        Echo { .. } => self.process_echo(command, &mut data.output_event_channel),
        Quit { .. } => self.process_quit(),
        _ => {
          if let Ok(action) = command.get_action() {
            self.process_action(action, &mut data.action_event_channel);
          } else {
            // println!("fail");
          }
        },
      }
    }
  }
}

mod action;
mod echo;
mod quit;
