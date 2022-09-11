use specs::prelude::*;
use std::fmt;

use crate::model::Direction;
use crate::model::Exit;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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
    trace_enter!();
    use Direction::*;
    let result = match direction {
      Northwest => self.northwest,
      North => self.north,
      Northeast => self.northeast,
      East => self.east,
      Southeast => self.southeast,
      South => self.south,
      Southwest => self.southwest,
      West => self.west,
      Up => self.up,
      Down => self.down,
      In => self.r#in,
      Out => self.out,
    };
    trace_exit!();
    result
  }

  #[named]
  pub fn set_exit(&mut self, direction: &Direction, exit: Option<Exit>) {
    trace_enter!();
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
    trace_exit!();
  }

  #[named]
  pub fn get_property_values(&self) -> Vec<Option<Exit>> {
    trace_enter!();
    let result = vec![
      self.north,
      self.northeast,
      self.east,
      self.southeast,
      self.south,
      self.southwest,
      self.west,
      self.northwest,
      self.up,
      self.down,
      self.r#in,
      self.out,
    ];
    trace_exit!();
    result
  }

  #[named]
  pub fn get_exits(&self) -> Vec<Exit> {
    trace_enter!();
    let mut result = Vec::new();
    for exit_option in self.get_property_values() {
      if let Some(exit) = exit_option {
        result.push(exit);
      }
    }
    trace_exit!();
    result
  }

  #[named]
  pub fn get_directions(&self) -> Vec<Direction> {
    trace_enter!();
    let result = self
      .get_exits()
      .iter()
      .map(|exit| exit.direction)
      .collect::<Vec<Direction>>();
    trace_exit!();
    result
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
    trace_enter!();
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
      }
    };
    let result = write!(formatter, "{}", string);
    trace_exit!();
    result
  }
}