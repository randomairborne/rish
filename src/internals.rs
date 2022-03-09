pub fn cd(args: Vec<&str>) {
    let path: String = match args.get(0) {
        Some(path) => path.to_string(),
        None => match std::env::var("HOME") {
            Ok(home) => home.to_string(),
            Err(_) => "/".to_string(),
        },
    };
    match std::env::set_current_dir(std::path::Path::new(&path)) {
        Ok(_) => {}
        Err(e) => println!("Failed to change directory: {}", e),
    }
}

pub fn echo(args: Vec<&str>) {
    println!("{}", args.join(" "));
}

pub fn help() {
    println!(
        r#"valkyrie_pilot's `{}-{}`
Usage:  {} [-h]
Builtins:
cd- Change working directory
set- Set environment variable
del- Delete environment variable 
help- show this message
exit- quit to control process
"#,
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_CRATE_NAME"),
    )
}

pub fn var(args: Vec<&str>) {
    let key: String = match args.get(0) {
        Some(key) => key.to_string(),
        None => {
            eprintln!("Missing environment variable name");
            return;
        }
    };
    let value: String = match args.get(1) {
        Some(value) => value.to_string(),
        None => {
            eprintln!("Missing environment variable value");
            return;
        }
    };
    if key.contains("=") || key.contains("\0") || key == String::from("") {
        eprintln!("Environment variable name contains =, null, or is empty");
        return;
    }
    if value.contains("\0") {
        eprintln!("Environment variable name contains =, null, or is empty");
        return;
    }
    std::env::set_var(key, value);
}

pub fn vardel(args: Vec<&str>) {
    let key: String = match args.get(0) {
        Some(key) => key.to_string(),
        None => {
            eprintln!("Missing environment variable name");
            return;
        }
    };
    if key.contains("=") || key.contains("\0") || key == String::from("") {
        eprintln!("Environment variable name contains =, null, or is empty");
        return;
    }
    std::env::remove_var(key);
}
