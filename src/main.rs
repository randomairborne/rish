#![feature(string_remove_matches)]
use rustyline::error::ReadlineError;
use std::io::Write;
mod cmd;
mod completion;
mod cwd;
mod internals;

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
    if rl.load_history(".rish.history").is_err() {}
    loop {
        let readline = rl.readline(format!("rish {} > ", cwd::get()).as_str());
        std::io::stdout().flush().expect("Failed to flush stdout");
        match readline {
            Ok(line) => {
                let mut args_with_cmd: Vec<&str> = line.split_whitespace().collect();
                let cmd = args_with_cmd.remove(0);
                let args = args_with_cmd;
                match cmd {
                    "exit" => break,
                    "cd" => internals::cd(args),
                    "echo" => internals::echo(args),
                    "help" => internals::help(),
                    _ => cmd::run(cmd, args),
                }
            rl.add_history_entry(line.as_str());
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            /*Err(ReadlineError::) => {
                println!("CTRL-D");
                break;
            }*/
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.append_history(".rish.history")
        .expect("Failed to save history!");
}

fn main() {
    rish_mainloop();
    println!("o/");
}
