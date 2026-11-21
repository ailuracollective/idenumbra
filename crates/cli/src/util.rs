//! util

use std::fs;
use std::path::{Path, PathBuf};

pub fn detect_source(explicit: Option<PathBuf>) -> Result<PathBuf, String> {
    if let Some(p) = explicit {
        return Ok(p);
    }
    if let Ok(env) = std::env::var("IDENUMBRA_SOURCE") {
        return Ok(PathBuf::from(env));
    }
    let mut cur = std::env::current_dir().map_err(|e| e.to_string())?;
    loop {
        if cur.join("crates").join("countries").is_dir() {
            return Ok(cur);
        }
        if !cur.pop() {
            break;
        }
    }
    Err(
        "could not locate idenumbra source repository; pass --source or set IDENUMBRA_SOURCE"
            .into(),
    )
}

pub fn copy_dir_all(src: &Path, dst: &Path, dry_run: bool) -> Result<usize, String> {
    if !dst.exists() && !dry_run {
        fs::create_dir_all(dst).map_err(|e| e.to_string())?;
    }
    let mut count = 0;
    for entry in fs::read_dir(src).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            count += copy_dir_all(&src_path, &dst_path, dry_run)?;
        } else {
            if !dry_run {
                if let Some(parent) = dst_path.parent() {
                    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
                }
                fs::copy(&src_path, &dst_path).map_err(|e| e.to_string())?;
            }
            let tag = if dry_run {
                console::style("[dry]").yellow().to_string()
            } else {
                console::style("wrote").green().to_string()
            };
            println!("  {} {}", tag, dst_path.display());
            count += 1;
        }
    }
    Ok(count)
}

pub fn manifest_path(project: &Path) -> PathBuf {
    project.join("idenumbra.toml")
}

pub fn read_installed(project: &Path) -> Vec<String> {
    let p = manifest_path(project);
    if !p.exists() {
        return vec![];
    }
    let content = fs::read_to_string(&p).unwrap_or_default();
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("installed") {
            if let (Some(s), Some(e)) = (line.find('['), line.find(']')) {
                let inner = &line[s + 1..e];
                return inner
                    .split(',')
                    .map(|x| x.trim().trim_matches('"').trim_matches('\'').to_string())
                    .filter(|x| !x.is_empty())
                    .collect();
            }
        }
    }
    vec![]
}

pub fn write_manifest(project: &Path, installed: &[String], dry_run: bool) -> Result<(), String> {
    let p = manifest_path(project);
    let mut sorted = installed.to_vec();
    sorted.sort();
    sorted.dedup();
    let body = format!(
        "[idenumbra]\nversion = \"{}\"\nsource = \"github:ailuracollective/idenumbra\"\n\n[paths]\nvalidators = \"src/idenumbra\"\n\n[countries]\ninstalled = [{}]\n",
        env!("CARGO_PKG_VERSION"),
        sorted.iter().map(|c| format!("\"{}\"", c.to_lowercase())).collect::<Vec<_>>().join(", ")
    );
    if dry_run {
        println!("[dry] would write {}", p.display());
        println!("{}", body);
    } else {
        fs::write(&p, body).map_err(|e| e.to_string())?;
        println!("  {} {}", console::style("wrote").green(), p.display());
    }
    Ok(())
}
