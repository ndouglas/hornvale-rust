use std::time::Duration;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum IoEvent {
  // Launch to initialize the application.
  Initialize,
  // Just take a little break.
  Sleep(Duration),
}
