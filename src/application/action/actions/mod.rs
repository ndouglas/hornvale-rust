use std::collections::HashMap;

use crate::application::input::key::Key;
use crate::application::Action;

#[derive(Default, Debug, Clone)]
pub struct Actions(Vec<Action>);

impl Actions {
  pub fn find(&self, key: Key) -> Option<&Action> {
    Action::iterator()
      .filter(|action| self.0.contains(action))
      .find(|action| action.keys().contains(&key))
  }

  pub fn actions(&self) -> &[Action] {
    self.0.as_slice()
  }
}

impl From<Vec<Action>> for Actions {
  fn from(actions: Vec<Action>) -> Self {
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
    let conflicts = map
      .iter()
      .filter(|(_, actions)| actions.len() > 1) 
      .map(|(key, actions)| {
        // At least two actions share same shortcut; panic!
        let actions = actions.iter().map(Action::to_string).collect::<Vec<_>>().join(", ");
        format!("Conflict key {} with actions {}", key, actions)
      })
      .collect::<Vec<_>>();
    if !conflicts.is_empty() {
      panic!("{}", conflicts.join("; "))
    }
    Self(actions)
  }
}

#[cfg(test)]
mod tests {
  use super::super::Action;
  use super::*;

  #[test]
  fn should_create_actions_from_vec() {
    let _actions: Actions = vec![Action::Quit, Action::Sleep].into();
  }

  #[test]
  #[should_panic]
  fn should_panic_when_create_actions_conflict_key() {
    let _actions: Actions = vec![Action::Quit, Action::Sleep, Action::Quit].into();
  }
}
