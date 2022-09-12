use specs::prelude::*;
use std::collections::VecDeque;
use std::sync::Mutex;

use crate::effects::Effect;
use crate::traits::Effectable;

lazy_static! {
  pub static ref EFFECT_QUEUE: Mutex<VecDeque<Effect>> = Mutex::new(VecDeque::new());
}

#[named]
pub fn enqueue_effect(effect: Effect) {
  EFFECT_QUEUE.lock().unwrap().push_back(effect);
}

#[named]
pub fn run_effect_queue(ecs: &mut World) {
  loop {
    let effect_option: Option<Effect> = EFFECT_QUEUE.lock().unwrap().pop_front();
    if let Some(effect) = effect_option {
      effect.execute(ecs);
    } else {
      break;
    }
  }
}
