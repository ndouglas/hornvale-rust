use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

use crate::action::Action;
use crate::component::components::*;
use crate::event::{ActionEvent, OutputEvent};
use crate::resource::resources::*;

pub struct ProcessActionSystem {
  pub reader_id: ReaderId<ActionEvent>,
}

impl ProcessActionSystem {}

#[derive(SystemData)]
pub struct ProcessActionSystemData<'a> {
  pub entities: Entities<'a>,
  pub has_description: ReadStorage<'a, HasDescription>,
  pub has_exits: ReadStorage<'a, HasExits>,
  pub has_name: ReadStorage<'a, HasName>,
  pub is_in_room: WriteStorage<'a, IsInRoom>,
  pub player_resource: Read<'a, PlayerResource>,
  pub action_event_channel: Write<'a, EventChannel<ActionEvent>>,
  pub output_event_channel: Write<'a, EventChannel<OutputEvent>>,
}

impl<'a> System<'a> for ProcessActionSystem {
  type SystemData = ProcessActionSystemData<'a>;

  #[named]
  fn run(&mut self, mut data: Self::SystemData) {
    trace_enter!();
    let events = data
      .action_event_channel
      .read(&mut self.reader_id)
      .into_iter()
      .map(|event| event.clone())
      .collect::<Vec<ActionEvent>>();
    let event_count = events.len();
    if event_count == 0 {
      return;
    }
    info!("Processing {} action event(s)...", event_count);
    for event in events.iter() {
      debug!("Processing next action event, {:?}", event);
      let ActionEvent { action } = event;
      use Action::*;
      match action {
        Look { entity } => {
          self.process_look(action.clone(), &mut data);
        },
        LookDirection { entity, direction } => self.process_look_direction(action.clone(), &mut data),
        MoveDirection { entity, direction } => self.process_move_direction(action.clone(), &mut data),
      }
    }
    trace_exit!();
  }
}

mod look;
mod look_direction;
mod move_direction;
