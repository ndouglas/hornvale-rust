#[macro_export]
macro_rules! eff_look {
  ($entity: expr) => {{
    Effect::Look(LookEffect { entity: $entity })
  }};
}

#[macro_export]
macro_rules! eff_move_entity {
  ($entity: expr, $from: expr, $to: expr) => {{
    Effect::MoveEntity(MoveEntityEffect {
      entity: $entity,
      from: $from,
      to: $to,
    })
  }};
}
