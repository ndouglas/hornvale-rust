use serde::*;
use specs::prelude::*;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum RunMode {
  Initial,
  MainGame,
}

impl RunMode {
  #[named]
  pub fn tick(self, _ecs: &mut World) -> Option<RunMode> {
    trace_enter!();
    use RunMode::*;
    let result = match self {
      Initial => Some(MainGame),
      MainGame => Some(MainGame),
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
    };
    trace_exit!();
    result
  }
}
