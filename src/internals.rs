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
    println!("There will be help here someday")
}
