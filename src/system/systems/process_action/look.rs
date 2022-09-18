use specs::prelude::*;

use crate::action::Action;
use crate::event::ActionEvent;

use super::*;

impl<'a> ProcessActionSystem {
  pub fn process_look(&mut self, action: Action, data: &mut ProcessActionSystemData<'a>) {
    if let Action::Look { entity } = action {
      if let Some(room_id) = get_current_room!(data, entity) {
        data.output_event_channel.single_write(OutputEvent {
          string: format_room!(data, room_id),
        });  
      }  
    }
  }
}
