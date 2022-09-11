use serde::*;
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
  Northwest,
  North,
  Northeast,
  East,
  Southeast,
  South,
  Southwest,
  West,
  Up,
  Down,
  In,
  Out,
}

impl Direction {
  #[named]
  pub fn get_inverse(&self) -> Direction {
    trace_enter!();
    use Direction::*;
    let result = match self {
      Northwest => Southeast,
      North => South,
      Northeast => Southwest,
      East => West,
      West => East,
      Southeast => Northwest,
      South => North,
      Southwest => Northeast,
      Up => Down,
      Down => Up,
      In => Out,
      Out => In,
    };
    trace_exit!();
    result
  }
}

impl fmt::Display for Direction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
