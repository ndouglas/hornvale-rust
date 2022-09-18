use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

use crate::command::Command;

use crate::event::{CommandEvent, InputEvent};
use crate::resource::*;

pub struct ProcessInputSystem {
  pub reader_id: ReaderId<InputEvent>,
}

impl ProcessInputSystem {}

#[derive(SystemData)]
pub struct ProcessInputSystemData<'a> {
  pub entities: Entities<'a>,
  pub input_event_channel: Read<'a, EventChannel<InputEvent>>,
  pub command_event_channel: Write<'a, EventChannel<CommandEvent>>,
  pub player_resource: Read<'a, PlayerResource>,
}

impl<'a> System<'a> for ProcessInputSystem {
  type SystemData = ProcessInputSystemData<'a>;

  #[named]
  fn run(&mut self, mut data: Self::SystemData) {
    trace_enter!();
    let input_events = data.input_event_channel.read(&mut self.reader_id).collect::<Vec<&InputEvent>>();
    let event_count = input_events.len();
    if event_count == 0 {
      return;
    }
    info!("Processing {} input event(s)...", event_count);
    let player = data.player_resource.0.unwrap();
    for event in input_events.iter() {
      if let Ok(command) = Command::from_str(&event.input, player) {
        data.command_event_channel.single_write(CommandEvent { command });
      } else {
        error!("Something was screwed up about {:?}", event);
      }
    }
    trace_exit!();
  }
}
