#[macro_export]
macro_rules! print_room {
  ($room: expr) => {{
    enq_event!(evt_print_room!($room));
  }};
}
