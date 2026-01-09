//! CLI module for downloading LedgerCloseMeta from Stellar public blockchain data.
//!
//! This module is only available when the `cli` feature is enabled.

use crate::ledger;
use clap::{Parser, ValueEnum};
use std::{ffi::OsString, fmt::Debug, io::Write};
use stellar_xdr::curr::{Limited, Limits, WriteXdr};

/// CLI tool to download LedgerCloseMeta from Stellar public blockchain data
#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = None,
    disable_help_subcommand = true,
)]
pub struct Root {
    /// Ledger sequence number to download
    #[arg(short, long)]
    sequence: u32,

    /// Output format
    #[arg(short, long, value_enum, default_value_t)]
    format: Format,

    /// Meta storage URL (SEP-54 compatible)
    #[arg(
        long,
        default_value = "https://aws-public-blockchain.s3.us-east-2.amazonaws.com/v1.1/stellar/ledgers/pubnet"
    )]
    meta_url: String,
}

#[derive(Default, Clone, ValueEnum, Debug)]
pub enum Format {
    #[default]
    Json,
    Xdr,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Clap(#[from] clap::Error),
    #[error("failed to download ledger meta: {0}")]
    Download(#[from] crate::Error),
    #[error("failed to serialize to JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("failed to write XDR: {0}")]
    Xdr(#[from] stellar_xdr::curr::Error),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl Root {
    /// Run the CLI command.
    ///
    /// ## Errors
    ///
    /// If the command fails to execute.
    pub fn run(&self) -> Result<(), Error> {
        let meta = ledger(&self.meta_url, self.sequence)?;

        match self.format {
            Format::Json => {
                let json = serde_json::to_string_pretty(&meta)?;
                println!("{}", json);
            }
            Format::Xdr => {
                let limits = Limits::none();
                let mut stdout = std::io::stdout();
                let mut limited_writer = Limited::new(&mut stdout, limits);
                meta.write_xdr(&mut limited_writer)?;
                stdout.flush()?;
            }
        }

        Ok(())
    }
}

/// Run the CLI with the given args.
///
/// ## Errors
///
/// If the input cannot be parsed or the command fails.
pub fn run<I, T>(args: I) -> Result<(), Error>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let root = Root::try_parse_from(args)?;
    root.run()
}
