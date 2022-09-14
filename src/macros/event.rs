#[macro_export]
macro_rules! evt_print_message {
  ($print_message_event: expr) => {{
    use crate::event::Event;
    Event::PrintMessage($print_message_event)
  }};
}

#[macro_export]
macro_rules! evt_print_room {
  ($entity: expr) => {{
    use crate::event::print_message::PrintMessageEvent;
    evt_print_message!(PrintMessageEvent::RoomDescription($entity))
  }};
}

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
    use crate::event::action_event::ActionEvent;
    use crate::event::action_event::CouldNotPerformActionEvent;
    evt_action!(ActionEvent::CouldNotPerform(CouldNotPerformActionEvent($action)))
  }};
}

#[macro_export]
macro_rules! evt_did_fail_to_perform_action {
  ($action: expr) => {{
    use crate::event::action_event::ActionEvent;
    use crate::event::action_event::DidFailToPerformActionEvent;
    evt_action!(ActionEvent::DidFailToPerform(DidFailToPerformActionEvent($action)))
  }};
}

#[macro_export]
macro_rules! evt_did_perform_action {
  ($action: expr) => {{
    use crate::event::action_event::ActionEvent;
    use crate::event::action_event::DidPerformActionEvent;
    evt_action!(ActionEvent::DidPerform(DidPerformActionEvent($action)))
  }};
}

#[macro_export]
macro_rules! evt_will_fail_to_perform_action {
  ($action: expr) => {{
    use crate::event::action_event::ActionEvent;
    use crate::event::action_event::WillFailToPerformActionEvent;
    evt_action!(ActionEvent::WillFailToPerform(WillFailToPerformActionEvent($action)))
  }};
}

#[macro_export]
macro_rules! evt_will_attempt_to_perform_action {
  ($action: expr) => {{
    use crate::event::action_event::ActionEvent;
    use crate::event::action_event::WillAttemptToPerformActionEvent;
    evt_action!(ActionEvent::WillAttemptToPerform(WillAttemptToPerformActionEvent(
      $action
    )))
  }};
}
