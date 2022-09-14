#[macro_export]
macro_rules! enq_message {
  ($message: expr) => {{
    crate::message::enqueue_message(format!("{}", $message))
  }};
}

#[macro_export]
macro_rules! enq_message_if {
  ($message: expr) => {{
    if let Some(inner) = $message {
      enq_message!(format!("{}", inner));
    }
  }};
}
