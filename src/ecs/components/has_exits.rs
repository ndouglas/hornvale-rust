use serde::*;
use specs::prelude::*;
use specs::prelude::*;
use specs_derive::Component;
use std::collections::HashMap;

use crate::model::Direction;
use crate::model::Exit;
use crate::model::Exits;
use crate::traits::WorldUsable;

#[derive(Component, Clone, Debug, Default, PartialEq)]
pub struct HasExits {
  pub exits: Exits,
}

pub trait HasExitsBuilder {
  fn has_exits(self) -> Self;
}

impl HasExitsBuilder for EntityBuilder<'_> {
  #[named]
  fn has_exits(self) -> Self {
    trace_enter!();
    let result = self.with(HasExits::default());
    trace_exit!();
    result
  }
}

pub trait HasExitsWorld {
  fn create_exit(&mut self, from: Entity, to: Entity, direction: &Direction, bidirectional: bool);
}

impl HasExitsWorld for World {
  #[named]
  fn create_exit(&mut self, from: Entity, to: Entity, direction: &Direction, bidirectional: bool) {
    trace_enter!();
    {
      let mut has_exits_storage = self.write_storage::<HasExits>();
      if let Some(HasExits { mut exits }) = has_exits_storage.get(from) {
        exits.set_exit(
          direction,
          Some(Exit {
            direction: direction.to_owned(),
            room_entity: to,
            is_passable: true,
          }),
        );
        has_exits_storage
          .insert(from, HasExits { exits })
          .expect("Unable to replace HasExits object.");
      }
    }
    if bidirectional {
      self.create_exit(to, from, &direction.get_inverse(), false);
    }
    trace_exit!();
  }
}
