use std::io::Error;

#[derive(Debug)]
pub struct ErrorEvent {
  pub error: Error,
}
