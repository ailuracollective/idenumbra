//! remove command.

use crate::util;
use std::path::PathBuf;

pub fn run(codes: Vec<String>, project: PathBuf, dry_run: bool) -> Result<(), String> {
    let mut installed = util::read_installed(&project);
    for code in &codes {
        let code_low = code.to_lowercase();
        let dst = project.join("src").join("idenumbra").join("countries").join(&code_low);
        if dst.exists() {
            if dry_run {
                println!("[dry] would remove {}", dst.display());
            } else {
                std::fs::remove_dir_all(&dst).map_err(|e| e.to_string())?;
                println!("  {} {}", console::style("removed").red(), dst.display());
            }
        } else {
            println!("  not installed: {}", code.to_uppercase());
        }
        installed.retain(|c| !c.eq_ignore_ascii_case(&code_low));
    }
    util::write_manifest(&project, &installed, dry_run)?;
    Ok(())
}
