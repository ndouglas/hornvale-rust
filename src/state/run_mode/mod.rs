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
    use RunMode::*;
    match self {
      Initial => Some(MainGame),
      MainGame => Some(MainGame),
      Quit => Some(Quit),
    }
  }

  #[named]
  pub fn should_continue(self) -> bool {
    use RunMode::*;
    match self {
      Initial => true,
      MainGame => true,
      Quit => false,
    }
  }

  #[named]
  pub fn should_maintain_ecs(self) -> bool {
    use RunMode::*;
    match self {
      Initial => false,
      MainGame => true,
      Quit => false,
    }
  }
}
