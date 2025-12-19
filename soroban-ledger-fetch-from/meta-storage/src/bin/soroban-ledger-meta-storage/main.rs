use soroban_ledger_fetch_from_meta_storage::cli;
use std::env;

fn main() {
    if let Err(e) = cli::run(env::args_os()) {
        match e {
            cli::Error::Clap(e) => e.exit(),
            _ => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    }
}
