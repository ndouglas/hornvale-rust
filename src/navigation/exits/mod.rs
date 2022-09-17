use std::fmt;

use crate::navigation::Direction;
use crate::navigation::Exit;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Exits {
  pub north: Option<Exit>,
  pub northeast: Option<Exit>,
  pub east: Option<Exit>,
  pub southeast: Option<Exit>,
  pub south: Option<Exit>,
  pub southwest: Option<Exit>,
  pub west: Option<Exit>,
  pub northwest: Option<Exit>,
  pub up: Option<Exit>,
  pub down: Option<Exit>,
  pub r#in: Option<Exit>,
  pub out: Option<Exit>,
}

impl Exits {
  #[named]
  pub fn get_exit(&self, direction: &Direction) -> Option<Exit> {
    use Direction::*;
    match direction {
      Northwest => self.northwest.clone(),
      North => self.north.clone(),
      Northeast => self.northeast.clone(),
      East => self.east.clone(),
      Southeast => self.southeast.clone(),
      South => self.south.clone(),
      Southwest => self.southwest.clone(),
      West => self.west.clone(),
      Up => self.up.clone(),
      Down => self.down.clone(),
      In => self.r#in.clone(),
      Out => self.out.clone(),
    }
  }

  #[named]
  pub fn set_exit(&mut self, direction: &Direction, exit: Option<Exit>) {
    use Direction::*;
    match direction {
      Northwest => self.northwest = exit,
      North => self.north = exit,
      Northeast => self.northeast = exit,
      East => self.east = exit,
      Southeast => self.southeast = exit,
      South => self.south = exit,
      Southwest => self.southwest = exit,
      West => self.west = exit,
      Up => self.up = exit,
      Down => self.down = exit,
      In => self.r#in = exit,
      Out => self.out = exit,
    }
  }

  #[named]
  pub fn get_property_values(&self) -> Vec<Option<Exit>> {
    vec![
      self.north.clone(),
      self.northeast.clone(),
      self.east.clone(),
      self.southeast.clone(),
      self.south.clone(),
      self.southwest.clone(),
      self.west.clone(),
      self.northwest.clone(),
      self.up.clone(),
      self.down.clone(),
      self.r#in.clone(),
      self.out.clone(),
    ]
  }

  #[named]
  pub fn get_exits(&self) -> Vec<Exit> {
    let mut result = Vec::new();
    for exit_option in self.get_property_values() {
      if let Some(exit) = exit_option {
        result.push(exit);
      }
    }
    result
  }

  #[named]
  pub fn get_directions(&self) -> Vec<Direction> {
    self
      .get_exits()
      .iter()
      .map(|exit| exit.direction.clone())
      .collect::<Vec<Direction>>()
  }
}

impl Default for Exits {
  fn default() -> Self {
    Exits {
      north: None,
      northeast: None,
      east: None,
      southeast: None,
      south: None,
      southwest: None,
      west: None,
      northwest: None,
      up: None,
      down: None,
      r#in: None,
      out: None,
    }
  }
}

impl fmt::Display for Exits {
  #[named]
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    let mut directions = self.get_directions();
    let string = match directions.len() {
      0 => "There are no visible exits.".into(),
      1 => format!("There is a visible exit to the {}.", directions.pop().unwrap()),
      2 => format!(
        "There are visible exits to the {} and {}.",
        directions.pop().unwrap(),
        directions.pop().unwrap()
      ),
      _ => {
        let last = directions.pop().unwrap();
        let others = directions
          .iter()
          .map(|d| d.get_lowercase())
          .collect::<Vec<String>>()
          .join(", ");
        format!("There are visible exits to the {}, and {}.", others, last)
      },
    };
    write!(formatter, "{}", string)
  }
}
