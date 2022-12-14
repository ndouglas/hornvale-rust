use specs::prelude::*;
use specs::shrev::EventChannel;

use crate::component::*;
use crate::event::OutputEvent;

pub struct ExperimentSystem {}

#[derive(SystemData)]
pub struct ExperimentSystemData<'a> {
  pub entities: Entities<'a>,
  pub has_description: ReadStorage<'a, HasDescription>,
  pub has_name: ReadStorage<'a, HasName>,
  pub output_event_channel: Write<'a, EventChannel<OutputEvent>>,
}

impl<'a> System<'a> for ExperimentSystem {
  type SystemData = ExperimentSystemData<'a>;

  fn run(&mut self, data: Self::SystemData) {
    for (_entity, _has_name, _has_description) in (&data.entities, &data.has_name, &data.has_description).join() {
      // println!("{}", has_name.0);
      // println!("{}", has_description.0);
    }
  }
}
