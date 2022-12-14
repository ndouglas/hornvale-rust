use colored::*;
use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

use crate::command::Command;

use crate::component::components::*;
use crate::event::{CommandEvent, InputEvent, OutputEvent};
use crate::resource::*;

pub struct ProcessInputSystem {
  pub reader_id: ReaderId<InputEvent>,
}

impl ProcessInputSystem {}

#[derive(SystemData)]
pub struct ProcessInputSystemData<'a> {
  pub entities: Entities<'a>,
  pub has_description: ReadStorage<'a, HasDescription>,
  pub has_exits: ReadStorage<'a, HasExits>,
  pub has_name: ReadStorage<'a, HasName>,
  pub is_an_object: ReadStorage<'a, IsAnObject>,
  pub is_in_room: ReadStorage<'a, IsInRoom>,
  pub input_event_channel: Read<'a, EventChannel<InputEvent>>,
  pub command_event_channel: Write<'a, EventChannel<CommandEvent>>,
  pub output_event_channel: Write<'a, EventChannel<OutputEvent>>,
  pub player_resource: Read<'a, PlayerResource>,
}

impl<'a> System<'a> for ProcessInputSystem {
  type SystemData = ProcessInputSystemData<'a>;

  #[named]
  fn run(&mut self, mut data: Self::SystemData) {
    trace_enter!();
    let input_events = data
      .input_event_channel
      .read(&mut self.reader_id)
      .map(|event| event.clone())
      .collect::<Vec<InputEvent>>();
    let event_count = input_events.len();
    if event_count == 0 {
      return;
    }
    info!("Processing {} input event(s)...", event_count);
    for event in input_events.iter() {
      data.output_event_channel.single_write(OutputEvent {
        string: format!("> {}", event.input.green()),
      });
      if let Ok(command) = self.get_command(&event.input, &mut data) {
        data.command_event_channel.single_write(CommandEvent { command });
      } else {
        error!("Something was screwed up about {:?}", event);
      }
    }
    trace_exit!();
  }
}

mod get_command;
mod match_visible_object;
