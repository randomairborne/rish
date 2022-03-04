#![feature(string_remove_matches)]
use rustyline::error::ReadlineError;
mod cmd;
mod completion;
mod cwd;
mod internals;
mod truncate_history;

fn rish_mainloop() {
    let file_completer = completion::MyHelper {
        completer: rustyline::completion::FilenameCompleter::new(),
        highlighter: rustyline::highlight::MatchingBracketHighlighter::new(),
        hinter: rustyline::hint::HistoryHinter {},
        colored_prompt: "".to_owned(),
        validator: rustyline::validate::MatchingBracketValidator::new(),
    };
    let config = rustyline::Config::builder()
        .history_ignore_space(true)
        .completion_type(rustyline::CompletionType::List)
        .build();
    let mut rl = rustyline::Editor::with_config(config);
    rl.set_helper(Some(file_completer));
    if rl.load_history("./.rish.history").is_err() {}
    loop {
        let prompt = format!("rish {} > ", cwd::get());
        rl.helper_mut()
            .expect("Failed to get readline helper")
            .colored_prompt = format!("\x1b[1;32m{}\x1b[0m", prompt);
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                let mut args_with_cmd: Vec<&str> = line.split_whitespace().collect();
                let cmd = args_with_cmd.remove(0);
                let args = args_with_cmd;
                rl.add_history_entry(line.as_str());

                match cmd {
                    "exit" => break,
                    "cd" => internals::cd(args),
                    "echo" => internals::echo(args),
                    "help" => internals::help(),
                    _ => cmd::run(cmd, args),
                }
            }
            Err(ReadlineError::Interrupted) => {
                eprintln!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                eprintln!("CTRL-D");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }
    }
    match rl.append_history(".rish.history") {
        Ok(_) => {}
        Err(e) => eprintln!("Error saving to history file: {}", e),
    }
    truncate_history::truncate_history();
}

fn main() {
    rish_mainloop();
    println!("o/");
}
