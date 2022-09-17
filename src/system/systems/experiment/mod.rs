use specs::prelude::*;
use specs::shrev::EventChannel;
use specs_derive::*;

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

  fn run(&mut self, mut data: Self::SystemData) {
    println!("Welll...");
    for i in 1..20 {
      data.output_event_channel.single_write(OutputEvent {
        string: "HMM....".to_string(),
      });  
    }
    for (_entity, _has_name, _has_description) in (&data.entities, &data.has_name, &data.has_description).join() {
      // println!("{}", has_name.0);
      // println!("{}", has_description.0);
    }
  }
}
