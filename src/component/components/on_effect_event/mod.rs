use crate::entity::Entity;
use crate::event::EffectEvent;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct OnEffectEvent {
  pub will_happen: Option<fn(EffectEvent)>,
  pub did_happen: Option<fn(EffectEvent)>,
}