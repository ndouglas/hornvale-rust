use specs::prelude::*;
use std::collections::VecDeque;

#[derive(Clone, Debug, Default, Hash, PartialEq)]
pub struct MessagesResource(pub VecDeque<String>);
