//! diff command.

use crate::commands::add;
use crate::util;
use std::path::PathBuf;

pub fn run(codes: Vec<String>, project: PathBuf) -> Result<(), String> {
    let to_check = if codes.is_empty() {
        util::read_installed(&project)
    } else {
        codes.iter().map(|c| c.to_lowercase()).collect()
    };
    add::run(to_check, project, None, true)
}
