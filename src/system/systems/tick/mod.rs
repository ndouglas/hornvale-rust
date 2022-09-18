use specs::prelude::*;

use crate::resource::*;

pub struct TickSystem {}

#[derive(SystemData)]
pub struct TickSystemData<'a> {
  pub entities: Entities<'a>,
  pub tick_resource: Write<'a, TickResource>,
}

impl<'a> System<'a> for TickSystem {
  type SystemData = TickSystemData<'a>;
  #[named]
  fn run(&mut self, mut data: Self::SystemData) {
    trace_enter!();
    data.tick_resource.0 = data.tick_resource.0.wrapping_add(1);
    trace_exit!();
  }
}
