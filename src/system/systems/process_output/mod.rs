use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

use crate::event::OutputEvent;
use crate::resource::*;

pub struct ProcessOutputSystem {
  pub reader_id: ReaderId<OutputEvent>,
}

impl ProcessOutputSystem {}

#[derive(SystemData)]
pub struct ProcessOutputSystemData<'a> {
  pub entities: Entities<'a>,
  pub messages_resource: Write<'a, MessagesResource>,
  pub output_event_channel: Read<'a, EventChannel<OutputEvent>>,
}

impl<'a> System<'a> for ProcessOutputSystem {
  type SystemData = ProcessOutputSystemData<'a>;
  #[named]
  fn run(&mut self, mut data: Self::SystemData) {
    trace_enter!();
    let output_events = data.output_event_channel.read(&mut self.reader_id).collect::<Vec<&OutputEvent>>();
    let event_count = output_events.len();
    if event_count == 0 {
      return;
    }
    info!("Processing {} output event(s)...", event_count);
    let messages = &mut data.messages_resource.0;
    for event in output_events.iter() {
      messages.push_back(event.string.clone());
    }
  }
}
