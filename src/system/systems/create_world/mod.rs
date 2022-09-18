use specs::prelude::*;

use crate::component::*;
use crate::navigation::*;
use crate::resource::*;

pub struct CreateWorldSystem {}

#[derive(SystemData)]
pub struct CreateWorldSystemData<'a> {
  pub entities: Entities<'a>,
  pub player_resource: Write<'a, PlayerResource>,
  pub has_description: WriteStorage<'a, HasDescription>,
  pub has_exits: WriteStorage<'a, HasExits>,
  pub has_name: WriteStorage<'a, HasName>,
  pub is_a_room: WriteStorage<'a, IsARoom>,
  pub is_an_object: WriteStorage<'a, IsAnObject>,
  pub is_in_room: WriteStorage<'a, IsInRoom>,
}

impl<'a> System<'a> for CreateWorldSystem {
  type SystemData = CreateWorldSystemData<'a>;

  fn run(&mut self, mut data: Self::SystemData) {
    let spawn_room = create_room!(data, "Spawn Room", "Dark olive trees crowd in on all sides, the air steams with the mist of a warm recent rain, midges hang in the air.");
    if let Some(player_id) = data.player_resource.0 {
      data.is_in_room.insert(player_id, IsInRoom(Some(spawn_room)));
    }
    let mushroom = create_object!(
      data,
      "Mushroom",
      "A speckled mushroom grows out of the sodden earth, on a long stalk.",
      spawn_room
    );
    let ne_room = create_room!(data, "Northeast Room", "This is the Northeastern Room.");
    let n_room = create_room!(data, "North Room", "This is the Northern Room.");
    let nw_room = create_room!(data, "Northwest Room", "This is the Northwestern Room.");
    let e_room = create_room!(data, "East Room", "This is the Eastern Room.");
    let w_room = create_room!(data, "West Room", "This is the Western Room.");
    let se_room = create_room!(data, "Southeast Room", "This is the Southeastern Room.");
    let s_room = create_room!(data, "South Room", "This is the Southern Room.");
    let sw_room = create_room!(data, "Southwest Room", "This is the Southwestern Room.");
    create_exit!(data, spawn_room, ne_room, &Direction::Northeast, true);
    create_exit!(data, spawn_room, n_room, &Direction::North, true);
    create_exit!(data, n_room, ne_room, &Direction::East, true);
    create_exit!(data, spawn_room, nw_room, &Direction::Northwest, true);
    create_exit!(data, n_room, nw_room, &Direction::West, true);
    create_exit!(data, spawn_room, e_room, &Direction::East, true);
    create_exit!(data, spawn_room, w_room, &Direction::West, true);
    create_exit!(data, spawn_room, se_room, &Direction::Southeast, true);
    create_exit!(data, spawn_room, s_room, &Direction::South, true);
    create_exit!(data, spawn_room, sw_room, &Direction::Southwest, true);
  }
}
