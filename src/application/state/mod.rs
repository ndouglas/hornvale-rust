use specs::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::dispatcher::get_new_dispatcher;
use crate::resource::TickResource;

pub mod components;
pub use components::*;
pub mod event_channels;
pub use event_channels::*;
pub mod resources;
pub use resources::*;

pub struct State<'a> {
  pub dispatcher: Arc<Mutex<Dispatcher<'a, 'a>>>,
  pub ecs: World,
  pub tick_counter: u64,
}

unsafe impl Send for State<'_> {}

impl State<'_> {
  #[named]
  pub fn new() -> Self {
    let mut ecs = World::new();
    insert_resources(&mut ecs);
    insert_event_channels(&mut ecs);
    register_components(&mut ecs);
    let dispatcher = Arc::new(Mutex::new(get_new_dispatcher(&mut ecs)));
    let tick_counter = 0;
    Self { ecs, dispatcher, tick_counter }
  }

  #[named]
  pub async fn tick(&mut self) {
    let mut dispatcher = self.dispatcher.lock().await;
    dispatcher.dispatch(&self.ecs);
    self.tick_counter = self.ecs.read_resource::<TickResource>().0;
  }

}
