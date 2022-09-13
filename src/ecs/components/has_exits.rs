use specs::prelude::*;

use specs_derive::Component;

use crate::model::Direction;
use crate::model::Exit;
use crate::model::Exits;

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
    self.with(HasExits::default())
  }
}

pub trait HasExitsWorld {
  fn create_exit(&mut self, from: Entity, to: Entity, direction: &Direction, bidirectional: bool);
}

impl HasExitsWorld for World {
  #[named]
  fn create_exit(&mut self, from: Entity, to: Entity, direction: &Direction, bidirectional: bool) {
    {
      let mut has_exits_storage = self.write_storage::<HasExits>();
      if let Some(HasExits { exits }) = has_exits_storage.get(from) {
        let mut new_exits = exits.clone();
        new_exits.set_exit(
          direction,
          Some(Exit {
            direction: direction.to_owned(),
            room_entity: to,
            is_passable: true,
          }),
        );
        has_exits_storage
          .insert(from, HasExits { exits: new_exits })
          .expect("Unable to replace HasExits object.");
      }
    }
    if bidirectional {
      if let Some(inverse) = &direction.get_inverse() {
        self.create_exit(to, from, inverse, false);
      }
    }
  }
}
