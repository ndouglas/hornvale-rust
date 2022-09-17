use colored::*;
use specs::prelude::*;
use std::str::FromStr;

use crate::action::Action;
use crate::navigation::Direction;

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Command {
  Echo { entity: Entity, string: String },
  Look { entity: Entity },
  MoveDirection { entity: Entity, direction: Direction },
  Quit { entity: Entity },
}

impl Command {
  #[named]
  pub fn from_str(string: &str, entity: Entity) -> Result<Self, ()> {
    let words: Vec<&str> = string.split_whitespace().collect();
    let first: String = words.get(0).unwrap_or(&"").to_string();
    let second: String = words.get(1).unwrap_or(&"").to_string();
    use Command::*;
    match first.as_str() {
      "echo" => Ok(Echo {
        entity,
        string: words[1..].join(" "),
      }),
      "look" | "l" => Ok(Look { entity }),
      "move" | "go" => match Direction::from_str(&second) {
        Ok(direction) => Ok(MoveDirection { entity, direction }),
        Err(_) => Err(()),
      },
      "quit" => Ok(Quit { entity }),
      other => match Direction::from_str(other) {
        Ok(direction) => Ok(MoveDirection { entity, direction }),
        Err(_) => Err(()),
      },
    }
  }

  #[named]
  pub fn get_action(&self) -> Result<Action, ()> {
    use Command::*;
    match self {
      Look { entity } => Ok(Action::Look { entity: *entity }),
      MoveDirection { entity, direction } => Ok(Action::MoveDirection { entity: *entity, direction: *direction }),
      _ => Err(()),
    }
  }
}
