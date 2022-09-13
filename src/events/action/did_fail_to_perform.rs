use specs::prelude::*;

use crate::events::EventMessageFormats;
use crate::events::EventVisibility;
use crate::traits::Eventable;

pub struct DidFailToPerformActionEvent {
  pub visibility: EventVisibility,
  pub formats: EventMessageFormats,
}

impl Eventable for DidFailToPerformActionEvent {
  /// Dispatch this event.
  fn dispatch(&self, _ecs: &mut World) {}
}
