use std::{fs, io::Read};

fn lines_from_file(filename: impl AsRef<std::path::Path>) -> Vec<String> {
    let mut file = fs::File::options()
        .read(true)
        .write(true)
        .open(filename)
        .expect("Failed to open file with `rw-`");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read to string");
    let not_a_vec_yet = contents.lines();
    let final_lines_as_strs = not_a_vec_yet.collect::<Vec<&str>>();
    let mut final_lines_as_strings = Vec::new();
    for item in final_lines_as_strs {
        final_lines_as_strings.push(item.to_string());
    }
    return final_lines_as_strings;
}

pub fn truncate_history() {
    let mut lines = lines_from_file("./.rish.history");
    if lines.len() > 10000 {
        lines.drain(0..(lines.len() - 10000));
    }
    fs::write("./.rish.history", lines.join("\n")).expect("Failed to write truncated history")
}
