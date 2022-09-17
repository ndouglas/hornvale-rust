use colored::*;

use specs::prelude::*;
use specs::shrev::EventChannel;
use std::collections::HashSet;

use crate::component::components::*;
use crate::dispatcher::get_new_dispatcher;
use crate::event::{ActionEvent, CommandEvent, InputEvent, OutputEvent};
use crate::resource::resources::*;

pub struct State<'a, 'b> {
  dispatcher: Dispatcher<'a, 'b>,
  ecs: World,
}

impl<'a, 'b> State<'a, 'b> {
  #[named]
  pub fn new() -> Self {
    let mut ecs = World::new();
    Self::insert_resources(&mut ecs);
    Self::insert_event_channels(&mut ecs);
    Self::register_components(&mut ecs);
    let dispatcher = get_new_dispatcher(&mut ecs);
    Self { ecs, dispatcher }
  }

  #[named]
  pub fn register_components(ecs: &mut World) {
    ecs.register::<HasDescription>();
    ecs.register::<HasName>();
  }

  #[named]
  pub fn insert_resources(ecs: &mut World) {
    ecs.insert(EditorResource(None));
    ecs.insert(PrinterResource(None));
    ecs.insert(PlayerResource(None));
    ecs.insert(PromptFormatResource("> ".blue().to_string()));
    ecs.insert(ShouldContinueResource(true));
    ecs.insert(ShouldPromptResource(true));
    ecs.insert(TickResource(0));
    ecs.insert(VisitedRoomsResource(HashSet::new()));
  }

  #[named]
  pub fn insert_event_channels(ecs: &mut World) {
    ecs.insert(EventChannel::<ActionEvent>::new());
    ecs.insert(EventChannel::<CommandEvent>::new());
    ecs.insert(EventChannel::<InputEvent>::new());
    ecs.insert(EventChannel::<OutputEvent>::new());
  }

  #[named]
  pub fn tick(&mut self) {
    self.dispatcher.dispatch(&self.ecs);
  }
}
