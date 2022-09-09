use specs::prelude::*;

use crate::ecs::components::*;

pub struct Visibility {}

impl<'a> System<'a> for Visibility {
  type SystemData = (
    Entities<'a>,
    ReadStorage<'a, HasName>,
    ReadStorage<'a, HasDescription>,
    ReadStorage<'a, IsAPlayer>,
    ReadStorage<'a, IsARoom>,
    ReadStorage<'a, IsInRoom>,
  );

  #[named]
  fn run(&mut self, data: Self::SystemData) {
    trace_enter!();
    let (entities, has_name_storage, has_description_storage, is_a_player_storage, is_a_room_storage, is_in_room_storage) = data;
    let is_in_room_components = (&entities, &is_a_player_storage, &is_in_room_storage)
      .join()
      .map(|(_entity, _, is_in_room)| is_in_room)
      .collect::<Vec<&IsInRoom>>();
    let room = is_in_room_components.first().unwrap().entity;
    let name = &has_name_storage.get(room).unwrap().name;
    let description = &has_description_storage.get(room).unwrap().description;
    print!("\n");
    print!("{}\n", name);
    print!("{}\n", description);
    print!("\n");
    trace_exit!();
  }
}
