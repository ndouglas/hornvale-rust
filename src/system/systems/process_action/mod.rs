use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

use crate::event::{ActionEvent, OutputEvent};

pub struct ProcessActionSystem {
  pub reader_id: ReaderId<ActionEvent>,
}

impl ProcessActionSystem {}

#[derive(SystemData)]
pub struct ProcessActionSystemData<'a> {
  pub entities: Entities<'a>,
  pub action_event_channel: Read<'a, EventChannel<ActionEvent>>,
  pub output_event_channel: Write<'a, EventChannel<OutputEvent>>,
}

impl<'a> System<'a> for ProcessActionSystem {
  type SystemData = ProcessActionSystemData<'a>;

  fn run(&mut self, mut data: Self::SystemData) {
    for event in data.action_event_channel.read(&mut self.reader_id) {
      data.output_event_channel.single_write(OutputEvent {
        string: format!("{:?}", event.action),
      });
    }
  }
}
