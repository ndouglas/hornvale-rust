use specs::prelude::*;

use crate::command::Command;
use crate::resource::ShouldContinueResource;

use super::*;

impl<'a> ProcessCommandSystem {
  pub fn process_quit(&mut self, should_continue_resource: &mut Write<'a, ShouldContinueResource>) {
    should_continue_resource.0 = false;
  }
}
