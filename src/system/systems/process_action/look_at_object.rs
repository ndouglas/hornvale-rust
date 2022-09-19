use crate::action::Action;

use super::*;

impl<'a> ProcessActionSystem {
  #[named]
  pub fn process_look_at_object(&mut self, action: Action, data: &mut ProcessActionSystemData<'a>) {
    trace_enter!();
    if let Action::LookAtObject { object, .. } = action {
      info!("Sending event (description of indicated object).");
      data.output_event_channel.single_write(OutputEvent {
        string: format!(
          "You look at the {}...",
          &data.has_name.get(object).unwrap().0.to_lowercase()
        ),
      });
      data.output_event_channel.single_write(OutputEvent {
        string: format!("{}", &data.has_description.get(object).unwrap().0),
      });
    }
    trace_exit!();
  }
}
