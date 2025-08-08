use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;
use std::path::{Path, PathBuf};
mod cli;
mod commands;
mod error;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::ConvertToGraphics {
            input_path,
            output_path,
        } => {
            let input_file_path = Path::new(&input_path).canonicalize()?;
            let output_directory_path = PathBuf::from(&output_path);

            commands::convert_to_graphics::run(input_file_path, output_directory_path)?;
        }
    }

    Ok(())
}
