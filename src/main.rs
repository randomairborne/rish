#![feature(string_remove_matches)]
use rustyline::error::ReadlineError;
use std::process::Stdio;
mod cwd;
mod internals;

fn run_command(cmd: &str, args: Vec<&str>) {
    match std::process::Command::new(cmd)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .output()
    {
        Ok(_) => {}
        Err(err) => eprintln!("{}", err),
    }
}

fn rish_mainloop() {
    let mut rl = rustyline::Editor::<()>::new();
    if rl.load_history(".rish.history").is_err() {}
    loop {
        let readline = rl.readline(format!("rish {} > ", cwd::get()).as_str());
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
                    _ => run_command(cmd, args),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(".rish.history")
        .expect("Failed to save history!");
}

fn main() {
    rish_mainloop();
    println!("o/");
}
