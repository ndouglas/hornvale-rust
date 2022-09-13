use specs::prelude::*;

use crate::events::EventMessageFormats;
use crate::events::EventVisibility;
use crate::traits::Eventable;

pub struct DidPerformActionEvent {
  pub visibility: EventVisibility,
  pub formats: EventMessageFormats,
}

impl Eventable for DidPerformActionEvent {
  /// Dispatch this event.
  fn dispatch(&self, _ecs: &mut World) {}
}
