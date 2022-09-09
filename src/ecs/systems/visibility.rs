use specs::prelude::*;

use crate::actions::*;
use crate::ecs::components::*;

pub struct Visibility {}

impl<'a> System<'a> for Visibility {
  type SystemData = (
    Entities<'a>,
    WriteStorage<'a, HasAction>,
    ReadStorage<'a, HasName>,
    ReadStorage<'a, HasDescription>,
    ReadStorage<'a, IsAPlayer>,
    ReadStorage<'a, IsARoom>,
    ReadStorage<'a, IsInRoom>,
  );

  #[named]
  fn run(&mut self, data: Self::SystemData) {
    trace_enter!();
    let (
      entities,
      mut has_action_storage,
      has_name_storage,
      has_description_storage,
      is_a_player_storage,
      is_a_room_storage,
      is_in_room_storage,
    ) = data;
    let mut entities_attempted_look: Vec<Entity> = Vec::new();
    let is_in_room_components = (
      &entities,
      &mut has_action_storage,
      &is_a_player_storage,
      &is_in_room_storage,
    )
      .join()
      .filter(|(_entity, has_action, _, _is_in_room_storage)| match has_action {
        HasAction {
          action: Action::Look(..),
        } => true,
        _ => false,
      })
      .map(|(entity, _, _, is_in_room)| (entity, is_in_room))
      .collect::<Vec<(Entity, &IsInRoom)>>();
    for (entity, is_in_room) in is_in_room_components.iter() {
      entities_attempted_look.push(*entity);
      let room = is_in_room.entity;
      let name = &has_name_storage.get(room).unwrap().name;
      let description = &has_description_storage.get(room).unwrap().description;
      print!("\n");
      print!("{}\n", name);
      print!("{}\n", description);
      print!("\n");
    }
    {
      for entity in entities_attempted_look.iter() {
        has_action_storage.remove(*entity);
      }
    }
    trace_exit!();
  }
}
