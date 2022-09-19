use std::str::FromStr;

use crate::navigation::Direction;

use super::*;

impl<'a> ProcessInputSystem {
  #[named]
  pub fn get_command(&mut self, input: &str, data: &mut ProcessInputSystemData<'a>) -> Result<Command, ()> {
    trace_enter!();
    let original_input = input.clone();
    let words: Vec<&str> = input.split_whitespace().collect();
    let first: String = words.get(0).unwrap_or(&"").to_string();
    let second: String = words.get(1).unwrap_or(&"").to_string();
    use Command::*;
    let entity = data.player_resource.0.unwrap();
    let result = match first.as_str() {
      "echo" => Ok(Echo {
        entity,
        string: words[1..].join(" "),
        original_input: original_input.to_string(),
      }),
      "look" | "l" => match second.as_str() {
        "" => Ok(Look {
          entity,
          original_input: original_input.to_string(),
        }),
        _ => match Direction::from_str(&second) {
          Ok(direction) => Ok(LookDirection {
            entity,
            direction,
            original_input: original_input.to_string(),
          }),
          Err(_) => {
            if let Ok(object) = self.match_visible_object(&words[1..].join(" "), data) {
              info!("Matched a visible object; using it...");
              Ok(LookAtObject {
                entity,
                object,
                original_input: original_input.to_string(),
              })
            } else {
              info!("Failed to match a visible object :(");
              Err(())
            }
          },
        },
      },
      "move" | "go" => match Direction::from_str(&second) {
        Ok(direction) => Ok(MoveDirection {
          entity,
          direction,
          original_input: original_input.to_string(),
        }),
        Err(_) => Err(()),
      },
      "quit" => Ok(Quit {
        entity,
        original_input: original_input.to_string(),
      }),
      other => match Direction::from_str(other) {
        Ok(direction) => Ok(MoveDirection {
          entity,
          direction,
          original_input: original_input.to_string(),
        }),
        Err(_) => Err(()),
      },
    };
    trace_exit!();
    result
  }
}
