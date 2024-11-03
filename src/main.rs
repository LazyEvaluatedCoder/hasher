mod error;
mod cli;
mod hasher;
mod multi_writer;

use error::Result;

use clap::Parser;

fn main() -> Result<()> {

    let args = cli::Cli::parse();

    let algorithm = &args.global_opts.algorithm;
    let all = args.global_opts.all;
    let verbose = args.global_opts.verbose;

    match args.command {
        cli::Commands::File { path } => {
            hasher::hash_file(&path, algorithm, all, verbose)?;
        },
        cli::Commands::Text { text } => {
            hasher::hash_text(&text, algorithm, all, verbose)?;
        },
        cli::Commands::Stdin => {
            hasher::hash_stdin(algorithm, all, verbose)?;
        }
    }

    Ok(())
}
