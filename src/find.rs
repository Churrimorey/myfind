use std::path::Path;

use regex::Regex;

use crate::walk_tree::walk_tree;

pub fn find<P: AsRef<Path>>(
    root: P,
    regex: &Regex,
    flag: i32,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut matches = Vec::new();
    walk_tree(root.as_ref(), regex, flag, &mut matches)?;
    Ok(matches)
}
