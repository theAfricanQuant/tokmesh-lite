use std::process::ExitCode;

use clap::Parser;
use tokmesh_lite::{
    AppError,
    cli::{Cli, Command, DataCommand, ProductCommand},
    data::csv::validate_data,
    load_manifest,
    product::{report::ValidationReport, validate::validate_product},
};

fn main() -> ExitCode {
    match run(Cli::parse()) {
        Ok(valid) => {
            if valid {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(1)
            }
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(2)
        }
    }
}

fn run(cli: Cli) -> Result<bool, AppError> {
    match cli.command {
        Command::Product { command } => match command {
            ProductCommand::Validate { manifest } => {
                let manifest = load_manifest(&manifest)?;
                let report = validate_product(&manifest);
                print_report(&report, cli.json)?;
                Ok(report.valid)
            }
            ProductCommand::Inspect { manifest } => {
                let manifest = load_manifest(&manifest)?;
                println!("{}", serde_json::to_string_pretty(&manifest)?);
                Ok(true)
            }
        },
        Command::Data { command } => match command {
            DataCommand::Validate { manifest, data } => {
                let manifest = load_manifest(&manifest)?;
                let mut report = validate_product(&manifest);
                if report.valid {
                    report.append(validate_data(&manifest, &data)?);
                }
                print_report(&report, cli.json)?;
                Ok(report.valid)
            }
        },
    }
}

fn print_report(report: &ValidationReport, json: bool) -> Result<(), AppError> {
    if json {
        println!("{}", serde_json::to_string_pretty(report)?);
    } else if report.valid {
        println!("VALID — no validation issues found");
    } else {
        println!("INVALID — {} issue(s)", report.issues.len());
        for issue in &report.issues {
            println!("- [{}] {}: {}", issue.code, issue.location, issue.message);
        }
    }
    Ok(())
}
