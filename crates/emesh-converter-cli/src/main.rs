use crate::arguments::{Arguments, Commands};
use clap::Parser;
use std::path::{Path, PathBuf};
mod arguments;
mod commands;

fn main() {
    tracing_subscriber::fmt::init();
    let arguments = Arguments::parse();

    match &arguments.command {
        Commands::ConvertToGraphics {
            input_path,
            output_path,
        } => {
            let input_file_path = Path::new(&input_path).canonicalize().unwrap();
            let output_directory_path = PathBuf::from(&output_path);

            commands::convert_to_graphics::run(input_file_path, output_directory_path);
        }
    }
}
