use std::process::Stdio;
pub fn run(cmd: &str, args: Vec<&str>) {
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
