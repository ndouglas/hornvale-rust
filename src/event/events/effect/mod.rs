use super::super::Eventable;
use crate::effect::Effect;
use crate::entity::Entity;
use crate::room::ROOM_ENTITIES;

pub enum EffectEvent {
  /// Sent when the effect will happen.
  WillHappen { effect: Effect, message: Option<String>, room: Entity },
  /// Sent when the effect did happen.
  DidHappen { effect: Effect, message: Option<String>, room: Entity },
}

impl Eventable for EffectEvent {
  /// Dispatch this event.
  fn dispatch(&self) {
    use EffectEvent::*;
    match self {
      WillHappen { effect, message, room } => {

      },
      DidHappen { effect, message, room } => {},
    }
  }
}
