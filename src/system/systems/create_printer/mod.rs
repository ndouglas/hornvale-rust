use specs::prelude::*;

use crate::resource::*;

pub struct CreatePrinterSystem {}

#[derive(SystemData)]
pub struct CreatePrinterSystemData<'a> {
  pub entities: Entities<'a>,
  pub editor_resource: Write<'a, EditorResource>,
  pub printer_resource: Write<'a, PrinterResource>,
}

impl<'a> System<'a> for CreatePrinterSystem {
  type SystemData = CreatePrinterSystemData<'a>;

  fn run(&mut self, mut data: Self::SystemData) {
    let editor = data.editor_resource.0.as_mut().unwrap();
    let printer = editor.create_external_printer().unwrap();
    data.printer_resource.0 = Some(Box::new(printer));
  }
}
