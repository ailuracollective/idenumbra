//! cli

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "idenumbra", version, about = "Install idenumbra validators into a project")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Add {
        codes: Vec<String>,
        #[arg(long, default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        source: Option<PathBuf>,
        #[arg(long)]
        dry_run: bool,
    },
    List {
        #[arg(long, default_value = ".")]
        path: PathBuf,
    },
    Remove {
        codes: Vec<String>,
        #[arg(long, default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        dry_run: bool,
    },
    Update {
        #[arg(long, default_value = ".")]
        path: PathBuf,
        #[arg(long)]
        dry_run: bool,
    },
    Diff {
        codes: Vec<String>,
        #[arg(long, default_value = ".")]
        path: PathBuf,
    },
}
