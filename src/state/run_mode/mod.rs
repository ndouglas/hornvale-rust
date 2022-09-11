use serde::*;
use specs::prelude::*;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum RunMode {
  Initial,
  MainGame,
  Quit,
}

impl RunMode {
  #[named]
  pub fn tick(self, _ecs: &mut World) -> Option<RunMode> {
    trace_enter!();
    use RunMode::*;
    let result = match self {
      Initial => Some(MainGame),
      MainGame => Some(MainGame),
      Quit => Some(Quit),
    };
    trace_exit!();
    result
  }

  #[named]
  pub fn should_continue(self) -> bool {
    trace_enter!();
    use RunMode::*;
    let result = match self {
      Initial => true,
      MainGame => true,
      Quit => false,
    };
    trace_exit!();
    result
  }

  #[named]
  pub fn should_maintain_ecs(self) -> bool {
    trace_enter!();
    use RunMode::*;
    let result = match self {
      Initial => false,
      MainGame => true,
      Quit => false,
    };
    trace_exit!();
    result
  }

}
