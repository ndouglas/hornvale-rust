#[macro_export]
macro_rules! eff_print_room {
  ($room: expr) => {{
    use crate::effect::print_room::PrintRoomEffect;
    use crate::effect::Effect;
    Effect::PrintRoom(PrintRoomEffect { room: $room })
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

#[macro_export]
macro_rules! eff_print_error {
  ($message: expr) => {{
    use crate::effect::*;
    Effect::PrintError(PrintErrorEffect { message: $message })
  }};
}

#[macro_export]
macro_rules! enq_effect {
  ($effect: expr) => {{
    crate::effect::enqueue_effect($effect)
  }};
}
