use serde::*;
use specs::prelude::*;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum RunMode {
  Initial,
  MainGame,
}

impl RunMode {
  pub fn tick(self, ecs: &mut World) -> Option<RunMode> {
    use RunMode::*;
    match self {
      Initial => Some(MainGame),
      MainGame => Some(MainGame),
    }
  }

  pub fn should_maintain_ecs(self) -> bool {
    use RunMode::*;
    match self {
      Initial => false,
      MainGame => true,
    }
  }
}
