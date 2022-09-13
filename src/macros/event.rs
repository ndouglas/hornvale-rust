#[macro_export]
macro_rules! evt_print_message {
  ($event: expr) => {{
    use crate::events::Event;
    Event::PrintMessage($event)
  }};
}

#[macro_export]
macro_rules! evt_print_room {
  ($entity: expr) => {{
    use crate::events::print_message::PrintMessageEvent;
    evt_print_message!(PrintMessageEvent::RoomDescription($entity))
  }};
}
