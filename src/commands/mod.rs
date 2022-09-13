use colored::*;
use specs::prelude::*;
use std::error::Error as ErrorTrait;
use std::fmt;
use std::str::FromStr;

use crate::actions::Action;
use crate::model::Direction;
use crate::traits::Commandable;

pub mod echo;
pub use echo::*;
pub mod look;
pub use look::*;
pub mod move_direction;
pub use move_direction::*;

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Command {
  Echo(EchoCommand),
  Look(LookCommand),
  MoveDirection(MoveDirectionCommand),
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
      "quit" => Ok(/*cmd_quit!(entity)*/ cmd_look!(entity)),
      "look" | "l" => Ok(cmd_look!(entity)),
      "move" | "go" => match Direction::from_str(&second) {
        Ok(direction) => Ok(cmd_move_to!(entity, direction)),
        Err(_) => Err(()),
      },
      other => {
        match Direction::from_str(other) {
          Ok(direction) => Ok(cmd_move_to!(entity, direction)),
          Err(_) => Err(()),
        }
      },
    }
  }
}
/*
"move" => match second.as_str() {
  "n" | "north" => Some(cmd_move_to!(entity, Direction::North)),
  "ne" | "northeast" => Some(cmd_move_to!(entity, Direction::Northeast)),
  "nw" | "northwest" => Some(cmd_move_to!(entity, Direction::Northwest)),
  "e" | "east" => Some(cmd_move_to!(entity, Direction::East)),
  "w" | "west" => Some(cmd_move_to!(entity, Direction::West)),
  "s" | "south" => Some(cmd_move_to!(entity, Direction::South)),
  "se" | "southeast" => Some(cmd_move_to!(entity, Direction::Southeast)),
  "sw" | "southwest" => Some(cmd_move_to!(entity, Direction::Southwest)),
  "up" => Some(cmd_move_to!(entity, Direction::Up)),
  "down" => Some(cmd_move_to!(entity, Direction::Down)),
  "in" => Some(cmd_move_to!(entity, Direction::In)),
  "out" => Some(cmd_move_to!(entity, Direction::Out)),
  &_ => {
    enq_message!(format!("{}", "What?".bright_red()));
    None
  }
},
"n" | "north" => Some(cmd_move_to!(entity, Direction::North)),
"ne" | "northeast" => Some(cmd_move_to!(entity, Direction::Northeast)),
"nw" | "northwest" => Some(cmd_move_to!(entity, Direction::Northwest)),
"e" | "east" => Some(cmd_move_to!(entity, Direction::East)),
"w" | "west" => Some(cmd_move_to!(entity, Direction::West)),
"s" | "south" => Some(cmd_move_to!(entity, Direction::South)),
"se" | "southeast" => Some(cmd_move_to!(entity, Direction::Southeast)),
"sw" | "southwest" => Some(cmd_move_to!(entity, Direction::Southwest)),
"up" => Some(cmd_move_to!(entity, Direction::Up)),
"down" => Some(cmd_move_to!(entity, Direction::Down)),
"in" => Some(cmd_move_to!(entity, Direction::In)),
"out" => Some(cmd_move_to!(entity, Direction::Out)),
"l" | "look" => Some(cmd_look!(entity)),
"echo" => Some(cmd_echo!(entity, words[1..].join(" "))),
"quit" => {
  enq_message!(format!("{}", "Quitting...".bright_blue()));
  state.quit();
  None
}
&_ => {
  enq_message!(format!("{}", "What?".bright_red()));
  None
}

fn from_str(input: &str) -> Result<Foo, Self::Err> {
  match input {
      "Bar"  => Ok(Foo::Bar),
      "Baz"  => Ok(Foo::Baz),
      "Bat"  => Ok(Foo::Bat),
      "Quux" => Ok(Foo::Quux),
      _      => Err(()),
  }
}
*/
impl Commandable for Command {
  #[named]
  fn execute(&self, ecs: &mut World) {
    use Command::*;
    match self {
      Echo(command) => command.execute(ecs),
      Look(command) => command.execute(ecs),
      MoveDirection(command) => command.execute(ecs),
    }
  }
}
