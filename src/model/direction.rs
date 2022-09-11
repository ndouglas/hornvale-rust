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

  #[named]
  pub fn get_name(&self) -> &str {
    trace_enter!();
    use Direction::*;
    let result = match self {
      Northwest => "Northwest",
      North => "North",
      Northeast => "Northeast",
      East => "East",
      West => "West",
      Southeast => "Southeast",
      South => "South",
      Southwest => "Southwest",
      Up => "Up",
      Down => "Down",
      In => "In",
      Out => "Out",
    };
    trace_exit!();
    result
  }

  #[named]
  pub fn get_lowercase(&self) -> String {
    trace_enter!();
    use Direction::*;
    let result = self.get_name().to_lowercase();
    trace_exit!();
    result
  }
}

impl fmt::Display for Direction {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "{}", self.get_lowercase())
  }
}
