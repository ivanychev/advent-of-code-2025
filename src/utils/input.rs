use log::info;
use std::path::PathBuf;

pub fn get_repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn get_input_path<S: AsRef<str>>(day: u32, part: Option<S>) -> PathBuf {
    let option_part = part.map(|p| format!("_{}", p.as_ref())).unwrap_or_default();
    let mut path = get_repo_root();
    path.push("inputs");
    path.push(format!("day{:02}{}.txt", day, option_part));

    path
}

pub fn read_input_lines<S: AsRef<str>>(day: u32, part: Option<S>) -> Vec<String> {
    let input_path = get_input_path(day, part);
    info!("Reading inputs from {}", input_path.display());
    std::fs::read_to_string(&input_path)
        .unwrap_or_else(|e| {
            panic!("Unable to read input file at {:?}: {e}", input_path);
        })
        .lines()
        .map(String::from)
        .filter(|line| !line.is_empty())
        .collect()
}
