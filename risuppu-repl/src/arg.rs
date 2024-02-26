use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Arg {
    /// Input files
    #[arg()]
    pub files: Vec<PathBuf>,

    /// Non-interaction mode
    #[arg(short, long, default_value_t = false)]
    pub interact: bool,

    /// Configuration file
    #[arg(short, long, env = "RISP_CONF")]
    pub configuration_file: Option<PathBuf>,
}
