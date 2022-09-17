use rustyline::{Editor, ExternalPrinter};
use specs::prelude::*;
use specs::shrev::EventChannel;

use crate::event::{ActionEvent, CommandEvent, InputEvent, OutputEvent};
use crate::resource::*;
use crate::system::*;

pub fn get_new_dispatcher<'a, 'b>(ecs: &mut World) -> Dispatcher<'a, 'b> {
  let process_input_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<InputEvent>>().register_reader();
    ProcessInputSystem { reader_id }
  };
  let process_command_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<CommandEvent>>().register_reader();
    ProcessCommandSystem { reader_id }
  };
  let process_action_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<ActionEvent>>().register_reader();
    ProcessActionSystem { reader_id }
  };
  let process_output_system = {
    let reader_id = ecs.fetch_mut::<EventChannel<OutputEvent>>().register_reader();
    ProcessOutputSystem { reader_id }
  };
  let dispatcher = DispatcherBuilder::new()
    .with(CreateEditorSystem {}, "create_editor", &[])
    .with(CreatePrinterSystem {}, "create_printer", &["create_editor"])
    .with(CreatePlayerSystem {}, "create_player", &[])
    .with(PromptSystem {}, "prompt", &["create_editor"])
    .with(ExperimentSystem {}, "experiment", &[])
    .with(process_output_system, "process_output", &[])
    .with(process_input_system, "process_input", &["prompt"])
    .with(process_command_system, "process_command", &["process_input"])
    .with(process_action_system, "process_action", &["process_command"])
    .with(
      TickSystem {},
      "tick",
      &["create_editor", "create_player", "process_action", "experiment"],
    )
    .build();
  dispatcher
}