use specs::prelude::*;

use crate::action::Action;
use crate::event::ActionEvent;

use super::*;

impl<'a> ProcessCommandSystem {
  pub fn process_action(&mut self, action: Action, action_event_channel: &mut Write<'a, EventChannel<ActionEvent>>) {
    action_event_channel.single_write(ActionEvent { action });
  }
}
