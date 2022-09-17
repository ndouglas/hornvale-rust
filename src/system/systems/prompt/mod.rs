use specs::prelude::*;
use specs::shrev::EventChannel;

use crate::event::InputEvent;
use crate::resource::*;

pub struct PromptSystem {}

#[derive(SystemData)]
pub struct PromptSystemData<'a> {
  pub entities: Entities<'a>,
  pub input_channel: Write<'a, EventChannel<InputEvent>>,
  pub prompt_format_resource: Read<'a, PromptFormatResource>,
  pub should_prompt_resource: Write<'a, ShouldPromptResource>,
  pub should_continue_resource: Write<'a, ShouldContinueResource>,
  pub editor_resource: Write<'a, EditorResource>,
}

impl<'a> System<'a> for PromptSystem {
  type SystemData = PromptSystemData<'a>;

  fn run(&mut self, mut data: Self::SystemData) {
    if !data.should_prompt_resource.0 {
      return;
    }
    if let None = &data.editor_resource.0 {
      return;
    }
    let input = {
      let editor = data.editor_resource.0.as_mut().unwrap();
      let format = &data.prompt_format_resource.0;
      editor.readline(format)
    };
    match input {
      Ok(line) => {
        data.input_channel.single_write(InputEvent { input: line.into() });
      },
      Err(_) => panic!("wut"),
    };
  }
}
