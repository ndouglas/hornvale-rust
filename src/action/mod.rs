use specs::prelude::*;
use std::collections::VecDeque;
use std::sync::Mutex;

pub mod actions;
pub use actions::*;

pub trait Actionable {
  fn attempt(&self);
}

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Action {
  Look(LookAction),
  MoveDirection(MoveDirectionAction),
}

impl Action {
  #[named]
  fn attempt(&self) {
    use Action::*;
    match self {
      Look(action) => action.attempt(),
      MoveDirection(action) => action.attempt(),
    }
  }
}

lazy_static! {
  pub static ref ACTION_QUEUE: Mutex<VecDeque<Action>> = Mutex::new(VecDeque::new());
}

#[named]
pub fn enqueue_action(action: Action) {
  ACTION_QUEUE.lock().unwrap().push_back(action);
}

#[named]
pub fn run_action_queue() {
  let actions = ACTION_QUEUE.lock().unwrap().drain(..).collect::<Vec<Action>>();
  for action in actions.iter() {
    action.attempt();
  }
}
