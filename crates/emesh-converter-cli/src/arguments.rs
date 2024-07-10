use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true)]
pub struct Arguments {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Convert a mesh to a graphics representation
    ConvertToGraphics {
        /// Path to the mesh dataset
        #[clap(short, long)]
        input_path: String,

        /// Path to the output directory
        #[clap(short, long)]
        output_path: String,
    },
}
