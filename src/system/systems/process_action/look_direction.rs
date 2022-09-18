use specs::prelude::*;

use crate::action::Action;

use super::*;

impl<'a> ProcessActionSystem {
  #[named]
  pub fn process_look_direction(&mut self, action: Action, data: &mut ProcessActionSystemData<'a>) {
    trace_enter!();
    if let Action::LookDirection { entity, direction } = action {
      if let Some(room_id) = get_current_room!(data, entity) {
        match get_exit_to!(data, room_id, &direction) {
          Some(exit) => {
            info!("Sending event (description of indicated room).");
            data.output_event_channel.single_write(OutputEvent {
              string: format!("You look to the {}...", &direction.get_lowercase()),
            });
            data.output_event_channel.single_write(OutputEvent {
              string: format_room!(data, exit.to),
            });
          },
          None => {
            data.output_event_channel.single_write(OutputEvent {
              string: "You are unable to look in that direction!".into(),
            });
          },
        }
      }
    }
    trace_exit!();
  }
}
