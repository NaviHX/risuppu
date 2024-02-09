use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Arg {
    /// Input files
    #[arg(short, long)]
    pub input_files: Vec<PathBuf>,

    /// Non-interaction mode
    #[arg(short, long, default_value_t = false)]
    pub interact: bool,
}
