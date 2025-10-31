use clap::{Parser, Subcommand, ValueHint};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Convert a mesh to a graphics representation
    ConvertToGraphics {
        /// Path to the mesh dataset
        #[clap(short, long, value_hint = ValueHint::FilePath)]
        input_path: PathBuf,

        /// Path to the output directory
        #[clap(short, long, value_hint = ValueHint::DirPath)]
        output_path: PathBuf,
    },
}
