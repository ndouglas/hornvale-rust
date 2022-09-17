use rustyline::ExternalPrinter;
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
  pub printer_resource: Write<'a, PrinterResource>,
  pub output_event_channel: Read<'a, EventChannel<OutputEvent>>,
}

impl<'a> System<'a> for ProcessOutputSystem {
  type SystemData = ProcessOutputSystemData<'a>;

  fn run(&mut self, mut data: Self::SystemData) {
    let printer_option = data.printer_resource.0.as_mut();
    if let Some(printer) = printer_option {
      for event in data.output_event_channel.read(&mut self.reader_id) {
        printer.print(format!("{:?}", event)).expect("External print failure");
      }
    }
  }
}
