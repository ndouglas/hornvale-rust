use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

use crate::event::ActionEvent;

pub struct ProcessActionSystem {
  pub reader_id: ReaderId<ActionEvent>,
}

impl ProcessActionSystem {}

#[derive(SystemData)]
pub struct ProcessActionSystemData<'a> {
  pub entities: Entities<'a>,
  pub action_event_channel: Read<'a, EventChannel<ActionEvent>>,
}

impl<'a> System<'a> for ProcessActionSystem {
  type SystemData = ProcessActionSystemData<'a>;

  fn run(&mut self, data: Self::SystemData) {
    for event in data.action_event_channel.read(&mut self.reader_id) {
      println!("{:?}", event);
    }
  }
}
