use specs::prelude::*;
use specs::shrev::EventChannel;

use crate::event::events::*;

  #[named]
  pub fn insert_event_channels(ecs: &mut World) {
    ecs.insert(EventChannel::<ActionEvent>::new());
    ecs.insert(EventChannel::<CommandEvent>::new());
    ecs.insert(EventChannel::<InputEvent>::new());
    ecs.insert(EventChannel::<OutputEvent>::new());
  }
