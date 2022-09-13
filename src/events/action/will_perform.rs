use specs::prelude::*;

use crate::events::EventMessageFormats;
use crate::events::EventVisibility;
use crate::traits::Eventable;

pub struct WillPerformActionEvent {
  pub visibility: EventVisibility,
  pub formats: EventMessageFormats,
}

impl Eventable for WillPerformActionEvent {
  /// Dispatch this event.
  fn dispatch(&self, _ecs: &mut World) {}
}
