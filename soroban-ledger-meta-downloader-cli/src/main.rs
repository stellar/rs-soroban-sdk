use clap::Parser;
use soroban_ledger_meta_downloader::{download_ledger_close_meta, S3Config};
use std::process;
use stellar_xdr::curr::{Limited, Limits, WriteXdr};

/// CLI tool to download LedgerCloseMeta from Stellar public blockchain data
#[derive(Parser)]
#[command(name = "soroban-ledger-meta-downloader")]
#[command(about = "Download ledger close meta data for a given ledger sequence")]
#[command(version, long_about = None)]
struct Args {
    /// Ledger sequence number to download
    #[arg(short, long)]
    sequence: u32,

    /// Output format: json or xdr (default: json)
    #[arg(short, long, default_value = "json")]
    format: String,

    /// S3 bucket name (default: aws-public-blockchain)
    #[arg(long, default_value = "aws-public-blockchain")]
    bucket: String,

    /// S3 region (default: us-east-2)
    #[arg(long, default_value = "us-east-2")]
    region: String,

    /// Root path prefix in the bucket (default: v1.1/stellar/ledgers/pubnet/)
    #[arg(long, default_value = "v1.1/stellar/ledgers/pubnet/")]
    prefix: String,
}

fn main() {
    let args = Args::parse();

    // Validate format
    if !matches!(args.format.as_str(), "json" | "xdr") {
        eprintln!(
            "Error: Invalid format '{}'. Supported formats: json, xdr",
            args.format
        );
        process::exit(1);
    }

    // Use region string directly
    let region = args.region.clone();

    // Create S3 config
    let config = S3Config {
        bucket: args.bucket,
        region,
        root_path: args.prefix,
    };

    // Download the ledger meta
    match download_ledger_close_meta(args.sequence, &config) {
        Ok(meta) => {
            match args.format.as_str() {
                "json" => match serde_json::to_string_pretty(&meta) {
                    Ok(json) => println!("{}", json),
                    Err(e) => {
                        eprintln!("Error: Failed to serialize to JSON: {}", e);
                        process::exit(1);
                    }
                },
                "xdr" => {
                    // Write raw XDR bytes
                    let limits = Limits::none(); // No limits for output
                    let mut stdout = std::io::stdout();
                    let mut limited_writer = Limited::new(&mut stdout, limits);
                    match meta.write_xdr(&mut limited_writer) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Error: Failed to write XDR: {}", e);
                            process::exit(1);
                        }
                    }
                }
                _ => unreachable!(), // We validated this above
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to download ledger meta: {}", e);
            process::exit(1);
        }
    }
}
