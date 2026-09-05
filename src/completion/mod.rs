use command::complete as complete_commands;
use file::complete as complete_files;
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::Validator;
use rustyline::{CompletionType, Config, Context, Editor, Helper};

mod command;
mod file;

pub struct CompleterHelper;

impl Completer for CompleterHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _context: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let before_cursor = &line[..pos];
        let token_start = before_cursor
            .char_indices()
            .rev()
            .find(|(_, character)| character.is_whitespace())
            .map(|(index, character)| index + character.len_utf8())
            .unwrap_or(0);
        let token = &before_cursor[token_start..];

        if token_start > 0 {
            let (start, candidates) = complete_files(token);
            return Ok((token_start + start, candidates));
        }

        Ok((0, complete_commands(token)))
    }
}

impl Hinter for CompleterHelper {
    type Hint = String;
}

impl Highlighter for CompleterHelper {}
impl Validator for CompleterHelper {}
impl Helper for CompleterHelper {}

pub fn new_editor() -> rustyline::Result<Editor<CompleterHelper, DefaultHistory>> {
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .build();

    let mut editor = Editor::with_config(config)?;
    editor.set_helper(Some(CompleterHelper));
    Ok(editor)
}
