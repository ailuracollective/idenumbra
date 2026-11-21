//! idenumbra CLI.

mod cli;
mod commands;
mod util;

use clap::Parser;
use cli::Cli;

fn main() {
    human_panic::setup_panic!();
    let cli = Cli::parse();
    let res = match cli.command {
        cli::Command::Add {
            codes,
            path,
            source,
            dry_run,
        } => commands::add::run(codes, path, source, dry_run),
        cli::Command::List { path } => commands::list::run(path),
        cli::Command::Remove {
            codes,
            path,
            dry_run,
        } => commands::remove::run(codes, path, dry_run),
        cli::Command::Update { path, dry_run } => commands::update::run(path, dry_run),
        cli::Command::Diff { codes, path } => commands::diff::run(codes, path),
    };
    if let Err(e) = res {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
