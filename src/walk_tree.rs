use std::{fs, path::Path};

use regex::Regex;

use colored::*;

pub fn walk_tree(
    dir: &Path,
    regex: &Regex,
    flag: i32,
    matches: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    //如果不是，应该怎么办？
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk_tree(&path, regex, flag, matches)?;
            } else if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                if regex.is_match(filename) {
                    matches.push(path.to_string_lossy().to_string());
                }
                if flag != 0 {
                    println!("{}", path.to_string_lossy().to_string().purple());
                }
            }
        }
    }
    Ok(())
}
