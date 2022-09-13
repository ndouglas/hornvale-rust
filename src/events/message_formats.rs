#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EventMessageFormats {
  /// The event as described to the actor.
  pub actor: Option<String>,
  /// The event as described to the target (if applicable).
  pub target: Option<String>,
  /// The event as described to an observer (if applicable).
  pub observer: Option<String>,
}
