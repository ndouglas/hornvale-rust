use rustyline::Editor;

#[derive(Debug, Default)]
pub struct EditorResource(pub Option<Editor<()>>);
