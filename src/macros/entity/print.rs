#[macro_export]
macro_rules! print_room {
  ($entity: expr) => {{
    use crate::events::Event;
    use crate::events::print_message::PrintMessageEvent;
    enq_event!(Event::PrintMessage(PrintMessageEvent::RoomDescription($entity)));
  }};
}
