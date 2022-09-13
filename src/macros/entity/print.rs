#[macro_export]
macro_rules! print_room {
  ($entity: expr) => {{
    enq_event!(evt_print_room!($entity));
  }};
}
