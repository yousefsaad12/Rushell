use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};

const BUILTINS: [&str; 5] = ["cd", "echo", "exit", "pwd", "type"];

pub struct BuiltinCompleter;

impl Completer for BuiltinCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _context: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let before_cursor = &line[..pos];
        if before_cursor.contains(char::is_whitespace) {
            return Ok((pos, Vec::new()));
        }

        let candidates = BUILTINS
            .iter()
            .filter(|builtin| builtin.starts_with(before_cursor))
            .map(|builtin| Pair {
                display: (*builtin).to_string(),
                replacement: (*builtin).to_string(),
            })
            .collect();

        Ok((0, candidates))
    }
}

impl Hinter for BuiltinCompleter {
    type Hint = String;
}

impl Highlighter for BuiltinCompleter {}
impl Validator for BuiltinCompleter {}
impl Helper for BuiltinCompleter {}

pub fn new_editor() -> rustyline::Result<Editor<BuiltinCompleter, DefaultHistory>> {
    let mut editor = Editor::new()?;
    editor.set_helper(Some(BuiltinCompleter));
    Ok(editor)
}
