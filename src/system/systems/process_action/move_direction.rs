use specs::prelude::*;

use crate::action::Action;
use crate::event::ActionEvent;

use super::*;

impl<'a> ProcessActionSystem {
  pub fn process_move_direction(&mut self, action: Action, data: &mut ProcessActionSystemData<'a>) {}
}
