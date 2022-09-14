use specs::prelude::*;
use std::collections::VecDeque;
use std::sync::Mutex;

use crate::state::STATE;

pub mod look;
pub use look::*;
pub mod move_direction;
pub use move_direction::*;

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
  loop {
    let action_option: Option<Action> = ACTION_QUEUE.lock().unwrap().pop_front();
    if let Some(action) = action_option {
      action.attempt();
    } else {
      break;
    }
  }
}
