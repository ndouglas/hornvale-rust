use crate::action::Action;
use crate::event::ActionEvent;

use super::*;

impl<'a> ProcessActionSystem {
  #[named]
  pub fn process_move_direction(&mut self, action: Action, data: &mut ProcessActionSystemData<'a>) {
    trace_enter!();
    if let Action::MoveDirection { entity, direction } = action {
      if let Some(room_id) = get_current_room!(data, entity) {
        match get_exit_to!(data, room_id, &direction) {
          Some(exit) => {
            if let Some(player_id) = data.player_resource.0 {
              data
                .is_in_room
                .insert(player_id, IsInRoom(Some(exit.to)))
                .expect("Unable to insert is-in-room for entity!");
              data.action_event_channel.single_write(ActionEvent {
                action: Action::Look { entity: player_id },
              });
            }
          },
          None => {
            data.output_event_channel.single_write(OutputEvent {
              string: "You are unable to move in that direction!".into(),
            });
          },
        }
      }
    }
    trace_exit!();
  }
}
