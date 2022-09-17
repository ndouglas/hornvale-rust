use rustyline::Editor;
use specs::prelude::*;

use crate::resource::*;

pub struct CreateEditorSystem {}

#[derive(SystemData)]
pub struct CreateEditorSystemData<'a> {
  pub entities: Entities<'a>,
  pub editor_resource: Write<'a, EditorResource>,
}

impl<'a> System<'a> for CreateEditorSystem {
  type SystemData = CreateEditorSystemData<'a>;

  fn run(&mut self, mut data: Self::SystemData) {
    if let None = data.editor_resource.0 {
      let editor = Editor::<()>::new().unwrap();
      data.editor_resource.0 = Some(editor);
    }
  }
}
