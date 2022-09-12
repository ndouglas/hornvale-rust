use serde::*;
use std::fmt;

use crate::model::OtherDirection;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
  Other(OtherDirection),
}

impl Direction {
  #[named]
  pub fn get_inverse(&self) -> Option<Direction> {
    trace_enter!();
    use Direction::*;
    let result = match self {
      Northwest => Some(Southeast),
      North => Some(South),
      Northeast => Some(Southwest),
      East => Some(West),
      West => Some(East),
      Southeast => Some(Northwest),
      South => Some(North),
      Southwest => Some(Northeast),
      Up => Some(Down),
      Down => Some(Up),
      In => Some(Out),
      Out => Some(In),
      Other(_) => None,
    };
    trace_exit!();
    result
  }

  #[named]
  pub fn get_name(&self) -> String {
    trace_enter!();
    use Direction::*;
    let result = match self {
      Northwest => "Northwest".to_owned(),
      North => "North".to_owned(),
      Northeast => "Northeast".to_owned(),
      East => "East".to_owned(),
      West => "West".to_owned(),
      Southeast => "Southeast".to_owned(),
      South => "South".to_owned(),
      Southwest => "Southwest".to_owned(),
      Up => "Up".to_owned(),
      Down => "Down".to_owned(),
      In => "In".to_owned(),
      Out => "Out".to_owned(),
      Other(other) => other.name.to_owned(),
    };
    trace_exit!();
    result
  }

  #[named]
  pub fn get_lowercase(&self) -> String {
    trace_enter!();
    use Direction::*;
    let result = match self {
      Other(other) => other.lowercase_name.to_owned(),
      _ => self.get_name().to_lowercase(),
    };
    trace_exit!();
    result
  }

  #[named]
  pub fn get_uppercase(&self) -> String {
    trace_enter!();
    use Direction::*;
    let result = match self {
      Other(other) => other.uppercase_name.to_owned(),
      _ => self.get_name().to_uppercase(),
    };
    trace_exit!();
    result
  }

  #[named]
  pub fn is_compass_direction(&self) -> bool {
    trace_enter!();
    use Direction::*;
    let result = match self {
      Up | Down | In | Out | Other(_) => false,
      _ => true,
    };
    trace_exit!();
    result
  }

}

impl fmt::Display for Direction {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "{}", self.get_lowercase())
  }
}
