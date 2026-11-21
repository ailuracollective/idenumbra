//! list command.

use crate::util;
use std::path::PathBuf;

pub fn run(project: PathBuf) -> Result<(), String> {
    let installed = util::read_installed(&project);
    if installed.is_empty() {
        println!("No validators installed.");
    } else {
        for c in installed {
            println!("{}", c.to_uppercase());
        }
    }
    Ok(())
}
