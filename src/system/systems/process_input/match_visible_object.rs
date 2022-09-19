use specs::prelude::*;

use super::*;

impl<'a> ProcessInputSystem {
  #[named]
  pub fn match_visible_object(&mut self, input: &str, data: &mut ProcessInputSystemData<'a>) -> Result<Entity, ()> {
    trace_enter!();
    info!(
      "Attempting to match a visible object with the descriptive text '{}'",
      input
    );
    let entity = data.player_resource.0.unwrap();
    let mut result = Err(());
    if let Some(current_room) = get_current_room!(data, entity) {
      info!("Examining visible objects in room {:?}", current_room);
      for (object_entity, _is_in_room, _is_an_object, _has_name, _has_description) in (
        &data.entities,
        &data.is_in_room,
        &data.is_an_object,
        &data.has_name,
        &data.has_description,
      )
        .join()
        .filter(|(_entity, is_in_room, _is_an_object, _has_name, _has_description)| is_in_room.0 == Some(current_room))
        .filter(|(_entity, _is_in_room, _is_an_object, has_name, _has_description)| {
          has_name.0.to_lowercase() == input.to_lowercase()
        })
      {
        info!("Found at least one candidate visible object: {:?}", _has_name);
        result = Ok(object_entity);
        break;
      }
    }
    trace_exit!();
    result
  }
}
