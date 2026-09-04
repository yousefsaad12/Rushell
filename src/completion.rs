use rustyline::completion::{ Completer, Pair };
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::Validator;
use rustyline::{ Context, Editor, Helper };
use crate::external::finder;

const BUILTINS: [&str; 5] = ["cd", "echo", "exit", "pwd", "type"];

pub struct BuiltinCompleter;

impl Completer for BuiltinCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _context: &Context<'_>
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let before_cursor = &line[..pos];
        if before_cursor.contains(char::is_whitespace) {
            return Ok((pos, Vec::new()));
        }

        let mut candidates: Vec<String> = BUILTINS.iter()
            .filter(|builtin| builtin.starts_with(before_cursor))
            .map(|builtin| (*builtin).to_string())
            .collect();
        candidates.extend(finder::find_executables(before_cursor));
        candidates.sort();
        candidates.dedup();

        let candidates = candidates
            .into_iter()
            .map(|candidate| Pair {
                display: candidate.clone(),
                replacement: candidate,
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
