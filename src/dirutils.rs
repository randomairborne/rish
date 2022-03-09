pub fn gethome() -> String {
    match std::env::var("HOME") {
        Ok(home) => home.to_string(),
        Err(_) => "/".to_string(),
    }
}

pub fn get() -> String {
    let current_directory_pathbuf = match std::env::current_dir() {
        Ok(path) => path,
        Err(_) => return "(Failed to get path)".to_string(),
    };
    let mut cwd_full = match current_directory_pathbuf.into_os_string().into_string() {
        Ok(path) => path,
        Err(_) => return "(path is invalid UTF8)".to_string(),
    };
    if cwd_full.contains(&gethome()) {
        cwd_full.remove_matches(&gethome());
        return format!("~{}", cwd_full);
    }
    cwd_full
}
