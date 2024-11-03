use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "Hasher", version)]
pub struct Cli {
    #[clap(flatten)]
    pub global_opts: GlobalOpts,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Args)]
pub struct GlobalOpts {
    /// Select hash algorithm
    #[arg(short, long, global = true, default_value = "sha256")]
    pub algorithm: String,

    /// Use all available hash algorithms
    #[arg(long, global = true, default_value_t = false)]
    pub all: bool,

    #[arg(short, long, global = true, default_value_t = false)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// File hashing
    File {
        path: PathBuf,
    },
    /// Text hashing
    Text {
        text: String,
    },
    /// Stdin hashing
    Stdin,
}