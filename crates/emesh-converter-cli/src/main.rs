use crate::cli::{Cli, Commands};
use anyhow::Result;
use clap::Parser;
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
            commands::convert_to_graphics::run(input_path.canonicalize()?, output_path)?;
        }
    }

    Ok(())
}
