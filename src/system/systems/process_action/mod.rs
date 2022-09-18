use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

use crate::action::Action;
use crate::component::components::*;
use crate::event::{ActionEvent, OutputEvent};

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
  pub is_in_room: ReadStorage<'a, IsInRoom>,
  pub action_event_channel: Read<'a, EventChannel<ActionEvent>>,
  pub output_event_channel: Write<'a, EventChannel<OutputEvent>>,
}

impl<'a> System<'a> for ProcessActionSystem {
  type SystemData = ProcessActionSystemData<'a>;

  fn run(&mut self, mut data: Self::SystemData) {
    let events = data
      .action_event_channel
      .read(&mut self.reader_id)
      .into_iter()
      .map(|event| event.clone())
      .collect::<Vec<ActionEvent>>();
    for event in events.iter() {
      let ActionEvent { action } = event;
      use Action::*;
      match action {
        Look { entity } => {
          self.process_look(action.clone(), &mut data);
        }
        MoveDirection { entity, direction } => self.process_move_direction(action.clone(), &mut data),
      }
    }
  }
}

mod look;
mod move_direction;
