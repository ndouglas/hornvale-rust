#[macro_export]
macro_rules! enq_action {
  ($action: expr) => {{
    crate::actions::enqueue_action($action)
  }};
}

#[macro_export]
macro_rules! enq_command {
  ($command: expr) => {{
    crate::commands::enqueue_command($command)
  }};
}

#[macro_export]
macro_rules! enq_effect {
  ($effect: expr) => {{
    crate::queue::enqueue_effect($effect)
  }};
}

#[macro_export]
macro_rules! enq_event {
  ($event: expr) => {{
    crate::queue::enqueue_event($event)
  }};
}

#[macro_export]
macro_rules! enq_message {
  ($message: expr) => {{
    crate::queue::enqueue_message(format!("{}", $message))
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
