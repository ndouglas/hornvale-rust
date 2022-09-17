use rustyline::Editor;
use specs::prelude::*;

#[derive(Debug, Default)]
pub struct EditorResource(pub Option<Editor<()>>);
