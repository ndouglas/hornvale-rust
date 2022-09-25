#[macro_export]
macro_rules! create_error {
  ($line_number: expr, $location: expr, $message: expr) => {{
    use std::io::{Error, ErrorKind};
    let location_str = if let Some(location) = $location {
      location
    } else {
      ""
    };
    Error::new(ErrorKind::Other, format!("[line {}] Error{}: {}", $line_number, location_str, $message))
  }};
}
