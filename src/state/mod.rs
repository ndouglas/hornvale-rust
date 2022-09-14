use colored::*;
use rustyline::Editor;
use specs::prelude::*;
use std::sync::Mutex;

use crate::ecs::components::*;
use crate::ecs::resources::*;
use crate::io::INPUT;
use crate::queue::*;
use crate::run_mode::RUN_MODE;
use crate::tick::TICK;

pub struct State {
  pub ecs: World,
  pub tick: u64,
}

lazy_static! {
  pub static ref STATE: Mutex<State> = Mutex::new(State::new());
}

impl State {
  #[named]
  pub fn new() -> Self {
    let mut ecs = World::new();
    register_components(&mut ecs);
    insert_resources(&mut ecs);
    State { ecs, tick: 0 }
  }
}
