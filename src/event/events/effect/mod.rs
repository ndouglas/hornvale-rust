use super::super::Eventable;
use crate::effect::Effect;
use crate::entity::Entity;
use crate::room::ROOM_ENTITIES;

pub enum EffectEvent {
  /// Sent when the effect will happen.
  EffectWillHappen {
    effect: Effect,
    message: Option<String>,
    room: Entity,
  },
  /// Sent when the effect did happen.
  EffectDidHappen {
    effect: Effect,
    message: Option<String>,
    room: Entity,
  },
}

impl Eventable for EffectEvent {
  /// Dispatch this event.
  fn dispatch(&self) {
    use EffectEvent::*;
    match self {
      EffectWillHappen { effect, message, room } => {},
      EffectDidHappen { effect, message, room } => {},
    }
  }
}
