use colored::*;
use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

use crate::event::{ErrorEvent, OutputEvent};

pub struct ProcessErrorSystem {
  pub reader_id: ReaderId<ErrorEvent>,
}

impl ProcessErrorSystem {}

#[derive(SystemData)]
pub struct ProcessErrorSystemData<'a> {
  pub entities: Entities<'a>,
  pub error_event_channel: Read<'a, EventChannel<ErrorEvent>>,
  pub output_event_channel: Write<'a, EventChannel<OutputEvent>>,
}

impl<'a> System<'a> for ProcessErrorSystem {
  type SystemData = ProcessErrorSystemData<'a>;
  #[named]
  fn run(&mut self, mut data: Self::SystemData) {
    trace_enter!();
    let error_events = data
      .error_event_channel
      .read(&mut self.reader_id)
      .collect::<Vec<&ErrorEvent>>();
    let event_count = error_events.len();
    if event_count == 0 {
      return;
    }
    info!("Processing {} error event(s)...", event_count);
    for event in error_events.iter() {
      debug!("Processing next error event {:?}", event);
      data.output_event_channel.single_write(OutputEvent {
        string: format!("{}", event.error.to_string().red()),
      });
    }
  }
}
