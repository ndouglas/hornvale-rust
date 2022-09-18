use specs::prelude::*;

use crate::action::Action;
use crate::event::ActionEvent;

use super::*;

impl<'a> ProcessCommandSystem {
  #[named]
  pub fn process_action(&mut self, action: Action, action_event_channel: &mut Write<'a, EventChannel<ActionEvent>>) {
    trace_enter!();
    action_event_channel.single_write(ActionEvent { action });
    trace_exit!();
  }
}
