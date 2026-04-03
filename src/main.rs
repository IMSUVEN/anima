use anyhow::Result;
use clap::Parser;

pub mod check;
mod cli;
pub mod config;
pub mod detect;
pub mod gc;
pub mod init;
pub mod plan;
pub mod score;
pub mod sprint;
pub mod status;
pub mod types;
pub mod upgrade;

fn main() -> Result<()> {
    let args = cli::Cli::parse();
    cli::dispatch(args)
}
