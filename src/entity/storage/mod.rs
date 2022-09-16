use super::Entity;
use crate::component::*;

zcomponents_storage!(EntityStorage<Entity>: {
  has_description: HasDescription,
  has_exits: HasExits,
  has_name: HasName,
  has_visited_rooms: HasVisitedRooms,
  is_a_player: IsAPlayer,
  is_a_room: IsARoom,
  is_an_object: IsAnObject,
  is_in_room: IsInRoom,
  on_action_event: OnActionEvent,
  on_effect_event: OnEffectEvent,
  on_get_description: OnGetDescription,
  on_get_name: OnGetName,
});
