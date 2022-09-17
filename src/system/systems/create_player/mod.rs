use specs::prelude::*;
use specs_derive::*;

use crate::component::*;
use crate::resource::*;

pub struct CreatePlayerSystem {}

#[derive(SystemData)]
pub struct CreatePlayerSystemData<'a> {
  pub entities: Entities<'a>,
  pub player_resource: Write<'a, PlayerResource>,
  pub has_description: WriteStorage<'a, HasDescription>,
  pub has_name: WriteStorage<'a, HasName>,
}

impl<'a> System<'a> for CreatePlayerSystem {
  type SystemData = CreatePlayerSystemData<'a>;

  fn run(&mut self, mut data: Self::SystemData) {
    if let None = data.player_resource.0 {
      let player = data.entities.create();
      data
        .has_name
        .insert(player, HasName("Player".into()))
        .expect("Unable to insert name for player!");
      data
        .has_description
        .insert(player, HasDescription("It's you, you idiot!".into()))
        .expect("Unable to insert description for player!");
      data.player_resource.0 = Some(player);
    }
  }
}
