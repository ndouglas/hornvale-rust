use specs::prelude::*;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EventVisibility {
  ActorOnly(Entity),
  TargetOnly(Entity),
  ActorAndTarget {
    actor: Entity,
    target: Entity,
  },
  ActorAndObservers {
    actor: Entity,
    observers: Vec<Entity>,
  },
  ActorTargetAndObservers {
    actor: Entity,
    target: Entity,
    observers: Vec<Entity>,
  },
}
