use colored::*;
use specs::prelude::*;
use std::error::Error as ErrorTrait;
use std::fmt;
use std::str::FromStr;

use crate::model::Direction;
use crate::traits::Commandable;

pub mod echo;
pub use echo::*;
pub mod look;
pub use look::*;
pub mod move_direction;
pub use move_direction::*;
pub mod quit;
pub use quit::*;

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Command {
  Echo(EchoCommand),
  Look(LookCommand),
  MoveDirection(MoveDirectionCommand),
  Quit(QuitCommand),
}

#[derive(Debug)]
pub enum Error {
  WTFError,
}

impl ErrorTrait for Error {}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::WTFError => write!(f, "{}", "What?".bright_red()),
    }
  }
}

impl Command {
  #[named]
  pub fn from_str(string: &str, entity: Entity) -> Result<Self, ()> {
    let words: Vec<&str> = string.split_whitespace().collect();
    let first: String = words.get(0).unwrap_or(&"").to_string();
    let second: String = words.get(1).unwrap_or(&"").to_string();
    match first.as_str() {
      "echo" => Ok(cmd_echo!(entity, words[1..].join(" "))),
      "quit" => Ok(cmd_quit!(entity)),
      "look" | "l" => Ok(cmd_look!(entity)),
      "move" | "go" => match Direction::from_str(&second) {
        Ok(direction) => Ok(cmd_move_to!(entity, direction)),
        Err(_) => Err(()),
      },
      other => match Direction::from_str(other) {
        Ok(direction) => Ok(cmd_move_to!(entity, direction)),
        Err(_) => Err(()),
      },
    }
  }
}

impl Commandable for Command {
  #[named]
  fn execute(&self) {
    use Command::*;
    match self {
      Echo(command) => command.execute(),
      Look(command) => command.execute(),
      MoveDirection(command) => command.execute(),
      Quit(command) => command.execute(),
    }
  }
}
