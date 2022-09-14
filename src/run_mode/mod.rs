use std::sync::Mutex;

lazy_static! {
  pub static ref RUN_MODE: Mutex<RunMode> = Mutex::new(RunMode::MainGame);
}

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum RunMode {
  MainGame,
  Quit,
}

impl RunMode {
  #[named]
  pub fn should_continue(self) -> bool {
    use RunMode::*;
    match self {
      Quit => false,
      _ => true,
    }
  }
}
