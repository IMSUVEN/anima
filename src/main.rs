use clap::Parser;

pub mod assess;
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
pub mod util;

fn main() -> std::process::ExitCode {
    let args = cli::Cli::parse();
    match cli::dispatch(args) {
        Ok(0) => std::process::ExitCode::SUCCESS,
        Ok(code) => std::process::ExitCode::from(code),
        Err(e) => {
            eprintln!("Error: {e:#}");
            std::process::ExitCode::from(1)
        }
    }
}
