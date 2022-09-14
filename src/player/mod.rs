use std::sync::Mutex;

use crate::entity::Entity;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player(pub Option<Entity>);

lazy_static! {
  pub static ref PLAYER: Mutex<Player> = Mutex::new(Player(None));
}
