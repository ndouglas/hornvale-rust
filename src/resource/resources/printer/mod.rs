use rustyline::{ Editor, ExternalPrinter };
use specs::prelude::*;
use std::fmt;

#[derive(Default)]
pub struct PrinterResource(pub Option<Box<dyn ExternalPrinter + Send + Sync>>);

impl fmt::Debug for PrinterResource {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "PrinterResource")
  }
}
