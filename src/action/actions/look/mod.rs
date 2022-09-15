use super::super::Actionable;
use crate::entity::Entity;

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub struct LookAction {
  pub entity: Entity,
}

impl Actionable for LookAction {
  #[named]
  fn attempt(&self) {
    if let Some(Some(room)) = get_current_room!(self.entity) {
      enq_effect!(eff_print_room!(room));
    }
  }
}