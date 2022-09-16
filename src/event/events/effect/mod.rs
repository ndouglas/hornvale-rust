use super::super::Eventable;
use crate::effect::Effect;
use crate::entity::Entity;
use crate::room::ROOM_ENTITIES;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum EffectEventType {
  EffectWillHappen,
  EffectDidHappen,
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct EffectEvent {
  pub r#type: EffectEventType,
  pub effect: Effect,
  pub message: Option<String>,
  pub room: Entity,
}

impl Eventable for EffectEvent {
  /// Dispatch this event.
  fn dispatch(&self) {
  }
}
