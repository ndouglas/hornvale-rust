#[macro_export]
macro_rules! enq_action {
  ($action: expr) => {
    crate::queue::enqueue_action($action);
  };
}

#[macro_export]
macro_rules! enq_command {
  ($command: expr) => {
    crate::queue::enqueue_command($command);
  };
}

#[macro_export]
macro_rules! enq_effect {
  ($effect: expr) => {
    crate::queue::enqueue_effect($effect);
  };
}

#[macro_export]
macro_rules! enq_message {
  ($message: expr) => {
    crate::queue::enqueue_message($message);
  };
}
