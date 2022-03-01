use std::io::Write;
use std::process::Stdio;
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
fn rash_mainloop() {
    loop {
        print!(
            "rash {} > ",
            std::env::current_dir().unwrap().as_path().display()
        );
        std::io::stdout()
            .flush()
            .expect("Failed to write to stdout");
        let line: String = text_io::read!("{}\n");
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
}

fn main() {
    rash_mainloop();
    println!("o/");
}
