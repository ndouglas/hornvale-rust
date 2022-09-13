use serde::*;
use std::fmt;
use std::str::FromStr;

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

impl FromStr for Direction {
  type Err = ();

  fn from_str(string: &str) -> Result<Self, ()> {
    use Direction::*;
    match string {
      "nw" | "northwest" => Ok(Northwest),
      "n" | "north" => Ok(North),
      "ne" | "northeast" => Ok(Northeast),
      "e" | "east" => Ok(East),
      "se" | "southeast" => Ok(Southeast),
      "s" | "south" => Ok(South),
      "sw" | "southwest" => Ok(Southwest),
      "w" | "west" => Ok(West),
      "up" => Ok(Up),
      "down" => Ok(Down),
      "in" => Ok(In),
      "out" => Ok(Out),
      _ => Err(()),
    }
  }
}

impl Direction {
  #[named]
  pub fn get_inverse(&self) -> Option<Direction> {
    use Direction::*;
    match self {
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
    }
  }

  #[named]
  pub fn get_name(&self) -> String {
    use Direction::*;
    match self {
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
    }
  }

  #[named]
  pub fn get_short_name(&self) -> String {
    use Direction::*;
    match self {
      Northwest => "nw".to_owned(),
      North => "n".to_owned(),
      Northeast => "ne".to_owned(),
      East => "e".to_owned(),
      West => "w".to_owned(),
      Southeast => "se".to_owned(),
      South => "s".to_owned(),
      Southwest => "sw".to_owned(),
      Up => "up".to_owned(),
      Down => "down".to_owned(),
      In => "in".to_owned(),
      Out => "out".to_owned(),
      Other(other) => other.name.to_owned(),
    }
  }

  #[named]
  pub fn get_lowercase(&self) -> String {
    use Direction::*;
    match self {
      Other(other) => other.lowercase_name.to_owned(),
      _ => self.get_name().to_lowercase(),
    }
  }

  #[named]
  pub fn get_uppercase(&self) -> String {
    use Direction::*;
    match self {
      Other(other) => other.uppercase_name.to_owned(),
      _ => self.get_name().to_uppercase(),
    }
  }

  #[named]
  pub fn is_compass_direction(&self) -> bool {
    use Direction::*;
    match self {
      Up | Down | In | Out | Other(_) => false,
      _ => true,
    }
  }
}

impl fmt::Display for Direction {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "{}", self.get_lowercase())
  }
}
