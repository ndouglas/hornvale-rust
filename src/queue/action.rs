use specs::prelude::*;
use std::collections::VecDeque;
use std::sync::Mutex;

use crate::actions::Action;
use crate::traits::Actionable;

lazy_static! {
  pub static ref ACTION_QUEUE: Mutex<VecDeque<Action>> = Mutex::new(VecDeque::new());
}

#[named]
pub fn enqueue_action(action: Action) {
  ACTION_QUEUE.lock().unwrap().push_back(action);
}

#[named]
pub fn run_action_queue(ecs: &mut World) {
  loop {
    let action_option: Option<Action> = ACTION_QUEUE.lock().unwrap().pop_front();
    if let Some(action) = action_option {
      if action.should_perform(ecs) {
        if action.can_perform(ecs) {
          action.perform(ecs);
        }
      }
    } else {
      break;
    }
  }
}
