#[macro_export]
macro_rules! add_command {
  ($ecs: expr, $entity: expr, $command: expr) => {{
    use crate::ecs::components::HasCommand;
    $ecs
      .write_storage::<HasCommand>()
      .insert($entity, HasCommand($command.clone()))
      .expect(format!("Could not insert command {:?} for entity {:?}", $command, $entity).as_str());
  }};
}
