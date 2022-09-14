use specs::prelude::*;
use std::collections::VecDeque;
use std::sync::Mutex;

pub mod effects;
pub use effects::*;

pub trait Effectable {
  /// Execute this effect.
  fn execute(&self) {
    todo!();
  }
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Effect {
  MoveEntity(MoveEntityEffect),
  PrintError(PrintErrorEffect),
  PrintRoom(PrintRoomEffect),
}

impl Effectable for Effect {
  #[named]
  fn execute(&self) {
    use Effect::*;
    match self {
      MoveEntity(effect) => effect.execute(),
      PrintError(effect) => effect.execute(),
      PrintRoom(effect) => effect.execute(),
    }
  }
}

lazy_static! {
  pub static ref EFFECT_QUEUE: Mutex<VecDeque<Effect>> = Mutex::new(VecDeque::new());
}

#[named]
pub fn enqueue_effect(effect: Effect) {
  EFFECT_QUEUE.lock().unwrap().push_back(effect);
}

#[named]
pub fn run_effect_queue() {
  let effects = EFFECT_QUEUE.lock().unwrap().drain(..).collect::<Vec<Effect>>();
  for effect in effects.iter() {
    effect.execute();
  }
}
