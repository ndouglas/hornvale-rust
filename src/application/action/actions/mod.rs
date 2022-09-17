use std::collections::HashMap;

use crate::application::Action;
use crate::application::input::key::Key;

/// The application should have some contextual actions.
#[derive(Default, Debug, Clone)]
pub struct Actions(Vec<Action>);

impl Actions {
  /// Given a key, find the corresponding action
  pub fn find(&self, key: Key) -> Option<&Action> {
    Action::iterator()
      .filter(|action| self.0.contains(action))
      .find(|action| action.keys().contains(&key))
  }

  /// Get contextual actions.
  /// (just for building a help view)
  pub fn actions(&self) -> &[Action] {
    self.0.as_slice()
  }
}

impl From<Vec<Action>> for Actions {
  /// Build contextual action
  ///
  /// # Panics
  ///
  /// If two actions have same key
  fn from(actions: Vec<Action>) -> Self {
    // Check key unicity
    let mut map: HashMap<Key, Vec<Action>> = HashMap::new();
    for action in actions.iter() {
      for key in action.keys().iter() {
        match map.get_mut(key) {
          Some(vec) => vec.push(*action),
          None => {
            map.insert(*key, vec![*action]);
          },
        }
      }
    }
    let errors = map
      .iter()
      .filter(|(_, actions)| actions.len() > 1) // at least two actions share same shortcut
      .map(|(key, actions)| {
        let actions = actions.iter().map(Action::to_string).collect::<Vec<_>>().join(", ");
        format!("Conflict key {} with actions {}", key, actions)
      })
      .collect::<Vec<_>>();
    if !errors.is_empty() {
      panic!("{}", errors.join("; "))
    }

    // Ok, we can create contextual actions
    Self(actions)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::super::Action;

  #[test]
  fn should_create_actions_from_vec() {
    let _actions: Actions = vec![
      Action::Quit,
      Action::Sleep,
      Action::IncrementDelay,
      Action::DecrementDelay,
    ]
    .into();
  }

  #[test]
  #[should_panic]
  fn should_panic_when_create_actions_conflict_key() {
    let _actions: Actions = vec![
      Action::Quit,
      Action::DecrementDelay,
      Action::Sleep,
      Action::IncrementDelay,
      Action::IncrementDelay,
      Action::Quit,
      Action::DecrementDelay,
    ]
    .into();
  }
}
