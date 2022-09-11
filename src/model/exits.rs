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
    };
    trace_exit!();
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
