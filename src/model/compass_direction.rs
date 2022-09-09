use serde::*;
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum CompassDirection {
  Northwest,
  North,
  Northeast,
  East,
  Southeast,
  South,
  Southwest,
  West,
}

impl CompassDirection {

  #[named]
  pub fn get_delta_xy(&self) -> (i32, i32) {
    trace_enter!();
    use CompassDirection::*;
    let result = match self {
      Northwest => (-1, -1),
      North => (0, -1),
      Northeast => (1, -1),
      East => (1, 0),
      Southeast => (1, 1),
      South => (0, 1),
      Southwest => (-1, 1),
      West => (-1, 0),
    };
    trace_exit!();
    result
  }

  #[named]
  pub fn get_inverse(&self) -> CompassDirection {
    trace_enter!();
    use CompassDirection::*;
    let result = match self {
      Northwest => Southeast,
      North => South,
      Northeast => Southwest,
      East => West,
      West => East,
      Southeast => Northwest,
      South => North,
      Southwest => Northeast,
    };
    trace_exit!();
    result
  }

}

impl fmt::Display for CompassDirection {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
