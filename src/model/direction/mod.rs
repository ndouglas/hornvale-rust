use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
  pub fn get_inverse(&self) -> Direction {
    use Direction::*;
    match self {
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
    }
  }

  #[named]
  pub fn get_name(&self) -> String {
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
    result.to_owned()
  }

  #[named]
  pub fn get_short_name(&self) -> String {
    use Direction::*;
    let result = match self {
      Northwest => "nw",
      North => "n",
      Northeast => "ne",
      East => "e",
      West => "w",
      Southeast => "se",
      South => "s",
      Southwest => "sw",
      Up => "up",
      Down => "down",
      In => "in",
      Out => "out",
    };
    result.to_owned()
  }

  #[named]
  pub fn get_lowercase(&self) -> String {
    self.get_name().to_lowercase()
  }

  #[named]
  pub fn get_uppercase(&self) -> String {
    self.get_name().to_uppercase()
  }

  #[named]
  pub fn is_compass_direction(&self) -> bool {
    use Direction::*;
    match self {
      Up | Down | In | Out => false,
      _ => true,
    }
  }
}

impl fmt::Display for Direction {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    write!(formatter, "{}", self.get_lowercase())
  }
}
