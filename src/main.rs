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

    if args.no_color {
        console::set_colors_enabled(false);
        console::set_colors_enabled_stderr(false);
    }

    if args.verbose && args.quiet {
        eprintln!("Error: --verbose and --quiet cannot be used together.");
        return std::process::ExitCode::from(1);
    }

    match cli::dispatch(args) {
        Ok(0) => std::process::ExitCode::SUCCESS,
        Ok(code) => std::process::ExitCode::from(code),
        Err(e) => {
            eprintln!("Error: {e:#}");
            let code = if e.downcast_ref::<config::ConfigError>().is_some() {
                3
            } else {
                1
            };
            std::process::ExitCode::from(code)
        }
    }
}
