#[macro_export]
macro_rules! evt_action {
  ($action_event: expr) => {{
    use crate::event::Event;
    Event::Action($action_event)
  }};
}

#[macro_export]
macro_rules! evt_could_not_perform_action {
  ($action: expr, $context: expr) => {{
    use crate::event::events::ActionEvent;
    use crate::event::events::CouldNotPerformAction;
    evt_action!(ActionEvent::CouldNotPerform(CouldNotPerformActionEvent($action)))
  }};
}

#[macro_export]
macro_rules! evt_did_fail_to_perform_action {
  ($action: expr) => {{
    use crate::event::events::ActionEvent;
    use crate::event::events::DidFailToPerformAction;
    evt_action!(ActionEvent::DidFailToPerform(DidFailToPerformActionEvent($action)))
  }};
}

#[macro_export]
macro_rules! evt_did_perform_action {
  ($action: expr, $message: expr, $room: expr) => {{
    use crate::event::events::ActionEvent;
    use crate::event::events::ActionEventType;
    evt_action!(ActionEvent::DidPerformAction {
      r#type: ActionEventType::DidPerformActionEvent($action),
      action: $action,
      message: $message,
      room: $room,
    })
  }};
}

#[macro_export]
macro_rules! evt_will_fail_to_perform_action {
  ($action: expr) => {{
    use crate::event::events::ActionEvent;
    use crate::event::events::WillFailToPerformAction;
    evt_action!(ActionEvent::WillFailToPerform(WillFailToPerformActionEvent($action)))
  }};
}

#[macro_export]
macro_rules! evt_will_attempt_to_perform_action {
  ($action: expr, $message: expr, $room: expr) => {{
    use crate::event::events::ActionEvent;
    use crate::event::events::ActionEventType;
    evt_action!(ActionEvent {
      r#type: ActionEventType::WillAttemptToPerformAction,
      action: $action,
      message: $message,
      room: $room,
    })
  }};
}

#[macro_export]
macro_rules! enq_event {
  ($event: expr) => {{
    crate::event::enqueue_event($event)
  }};
}

#[macro_export]
macro_rules! dispatch_action_event {
  ($event: expr, $property: ident) => {{
    use std::collections::HashSet;
    use crate::component::*;
    use crate::entity::ENTITIES;
    let entity_ids = {
      let entities = ENTITIES.lock().unwrap();
      let entities_in_a_room = entities
        .is_in_room
        .ids_collected()
        .into_iter()
        .collect::<HashSet<Entity>>();
      let observant_entities = entities
        .on_action_event
        .ids_collected()
        .into_iter()
        .collect::<HashSet<Entity>>();
      entities_in_a_room
        .intersection(&observant_entities)
        .into_iter()
        .map(|entity| *entity)
        .filter(|entity| IsInRoom(Some($event.room)) == *entities.is_in_room.get(*entity))
        .collect::<Vec<Entity>>()
    };
    let mut components = Vec::new();
    {
      let entities = ENTITIES.lock().unwrap();
      for entity in entity_ids.iter() {
        let component = entities.on_action_event.get(*entity);
        if let Some(function) = component.$property {
          components.push(component.clone());
        }
      }
    }
    for component in components.iter() {
      component.$property.unwrap()($event.clone());
    }
    {
      let entities = ENTITIES.lock().unwrap();
      if let Some(room_component) = ENTITIES.lock().unwrap().on_action_event.get_opt($event.room) {
        if let Some(function) = room_component.$property {
          function($event.clone());
        }
      }
    }
  }};
}
