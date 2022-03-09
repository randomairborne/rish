use std::process::Stdio;
pub fn run(cmd: &str, args: Vec<&str>) {
    match std::process::Command::new(cmd)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
    {
        Ok(_) => {}
        Err(err) => eprintln!("{}", err),
    }
}
