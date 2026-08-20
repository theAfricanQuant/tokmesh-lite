use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "tokmesh-lite", version, about)]
pub struct Cli {
    /// Print machine-readable JSON instead of a human report.
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Validate or inspect a data-product manifest.
    Product {
        #[command(subcommand)]
        command: ProductCommand,
    },
    /// Validate a dataset against its product manifest.
    Data {
        #[command(subcommand)]
        command: DataCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum ProductCommand {
    /// Validate the manifest without reading a dataset.
    Validate { manifest: PathBuf },
    /// Print the parsed, normalized manifest.
    Inspect { manifest: PathBuf },
}

#[derive(Debug, Subcommand)]
pub enum DataCommand {
    /// Validate CSV data against the manifest's schema and quality rules.
    Validate { manifest: PathBuf, data: PathBuf },
}
