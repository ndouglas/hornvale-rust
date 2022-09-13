use specs::prelude::*;

use crate::events::EventMessageFormats;
use crate::events::EventVisibility;
use crate::traits::Eventable;

pub struct WillFailToPerformActionEvent {
  pub visibility: EventVisibility,
  pub formats: EventMessageFormats,
}

impl Eventable for WillFailToPerformActionEvent {
  /// Dispatch this event.
  fn dispatch(&self, _ecs: &mut World) {}
}
