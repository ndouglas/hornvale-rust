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
  pub output_event_channel: Read<'a, EventChannel<OutputEvent>>,
}

impl<'a> System<'a> for ProcessOutputSystem {
  type SystemData = ProcessOutputSystemData<'a>;

  fn run(&mut self, mut data: Self::SystemData) {
  }
}
