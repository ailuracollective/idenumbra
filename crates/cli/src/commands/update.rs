//! update command.

use crate::commands::add;
use crate::util;
use std::path::PathBuf;

pub fn run(project: PathBuf, dry_run: bool) -> Result<(), String> {
    let installed = util::read_installed(&project);
    if installed.is_empty() {
        println!("Nothing to update.");
        return Ok(());
    }
    let source = match fetch_source_tarball(env!("CARGO_PKG_VERSION")) {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!(
                "warning: could not fetch remote source ({}); using local",
                e
            );
            None
        }
    };
    add::run(installed, project, source, dry_run)
}

fn fetch_source_tarball(version: &str) -> Result<PathBuf, String> {
    let url = format!(
        "https://github.com/ailuracollective/idenumbra/archive/refs/tags/v{}.tar.gz",
        version
    );
    let resp = reqwest::blocking::get(&url).map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("HTTP {}", resp.status()));
    }
    let bytes = resp.bytes().map_err(|e| e.to_string())?;
    let tmp = std::env::temp_dir().join(format!("idenumbra-src-{}", version));
    if tmp.exists() {
        std::fs::remove_dir_all(&tmp).ok();
    }
    std::fs::create_dir_all(&tmp).map_err(|e| e.to_string())?;
    let dec = flate2::read::GzDecoder::new(&bytes[..]);
    let mut archive = tar::Archive::new(dec);
    archive.unpack(&tmp).map_err(|e| e.to_string())?;
    for entry in std::fs::read_dir(&tmp).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.path().is_dir() {
            return Ok(entry.path());
        }
    }
    Err("extracted archive had no root directory".into())
}
