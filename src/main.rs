use clap::Parser;
use std::io;
use std::process::ExitCode;

use rs_img2pnm::stdin2img2pnm;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Max input bytes to read from stdin (e.g., 1048576 for 1MB)
    #[arg(long, default_value_t = 1048576)]
    max_input_bytes: u64,
}

fn sub() -> Result<(), io::Error> {
    let cli = Cli::parse();
    stdin2img2pnm(cli.max_input_bytes)
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
