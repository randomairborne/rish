#![feature(string_remove_matches)]
mod cmd;
mod completion;
mod configs;
mod dirutils;
mod internals;
mod truncate_history;

const HISTORY_FILE: &str = ".rish.history";

fn rish_mainloop() {
    let file_completer = completion::MyHelper {
        completer: rustyline::completion::FilenameCompleter::new(),
        highlighter: rustyline::highlight::MatchingBracketHighlighter::new(),
        hinter: (),
        colored_prompt: "".to_owned(),
        validator: rustyline::validate::MatchingBracketValidator::new(),
    };
    let lineread_config = rustyline::Config::builder()
        .history_ignore_space(true)
        .completion_type(rustyline::CompletionType::List)
        .build();
    let mut rl = rustyline::Editor::with_config(lineread_config);
    rl.set_helper(Some(file_completer));
    if rl
        .load_history(&(dirutils::gethome() + HISTORY_FILE))
        .is_err()
    {}
    let config = configs::load();
    loop {
        let prompt = format!("rish {} > ", dirutils::get());
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
                    "set" => internals::var(args),
                    "var" => internals::var(args),
                    "del" => internals::vardel(args),
                    _ => cmd::run(cmd, args),
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                eprintln!("CTRL-C");
                break;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                eprintln!("CTRL-D");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }
    }
    match rl.append_history(&(dirutils::gethome() + HISTORY_FILE)) {
        Ok(_) => {}
        Err(e) => eprintln!("Error saving to history file: {}", e),
    }
    truncate_history::truncate_history();
}

fn main() {
    rish_mainloop();
    println!("o/");
}
